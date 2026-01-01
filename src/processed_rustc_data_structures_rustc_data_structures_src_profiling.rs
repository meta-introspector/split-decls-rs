/* FP:profiling.rs-0001 */ // # Rust Compiler Self-Profiling
/* FP:profiling.rs-0002 */ //
/* FP:profiling.rs-0003 */ // This module implements the basic framework for the compiler's self-
/* FP:profiling.rs-0004 */ // profiling support. It provides the `SelfProfiler` type which enables
/* FP:profiling.rs-0005 */ // recording "events". An event is something that starts and ends at a given
/* FP:profiling.rs-0006 */ // point in time and has an ID and a kind attached to it. This allows for
/* FP:profiling.rs-0007 */ // tracing the compiler's activity.
/* FP:profiling.rs-0008 */ //
/* FP:profiling.rs-0009 */ // Internally this module uses the custom tailored [measureme][mm] crate for
/* FP:profiling.rs-0010 */ // efficiently recording events to disk in a compact format that can be
/* FP:profiling.rs-0011 */ // post-processed and analyzed by the suite of tools in the `measureme`
/* FP:profiling.rs-0012 */ // project. The highest priority for the tracing framework is on incurring as
/* FP:profiling.rs-0013 */ // little overhead as possible.
/* FP:profiling.rs-0014 */ //
/* FP:profiling.rs-0015 */ //
/* FP:profiling.rs-0016 */ // ## Event Overview
/* FP:profiling.rs-0017 */ //
/* FP:profiling.rs-0018 */ // Events have a few properties:
/* FP:profiling.rs-0019 */ //
/* FP:profiling.rs-0020 */ // - The `event_kind` designates the broad category of an event (e.g. does it
/* FP:profiling.rs-0021 */ //   correspond to the execution of a query provider or to loading something
/* FP:profiling.rs-0022 */ //   from the incr. comp. on-disk cache, etc).
/* FP:profiling.rs-0023 */ // - The `event_id` designates the query invocation or function call it
/* FP:profiling.rs-0024 */ //   corresponds to, possibly including the query key or function arguments.
/* FP:profiling.rs-0025 */ // - Each event stores the ID of the thread it was recorded on.
/* FP:profiling.rs-0026 */ // - The timestamp stores beginning and end of the event, or the single point
/* FP:profiling.rs-0027 */ //   in time it occurred at for "instant" events.
/* FP:profiling.rs-0028 */ //
/* FP:profiling.rs-0029 */ //
/* FP:profiling.rs-0030 */ // ## Event Filtering
/* FP:profiling.rs-0031 */ //
/* FP:profiling.rs-0032 */ // Event generation can be filtered by event kind. Recording all possible
/* FP:profiling.rs-0033 */ // events generates a lot of data, much of which is not needed for most kinds
/* FP:profiling.rs-0034 */ // of analysis. So, in order to keep overhead as low as possible for a given
/* FP:profiling.rs-0035 */ // use case, the `SelfProfiler` will only record the kinds of events that
/* FP:profiling.rs-0036 */ // pass the filter specified as a command line argument to the compiler.
/* FP:profiling.rs-0037 */ //
/* FP:profiling.rs-0038 */ //
/* FP:profiling.rs-0039 */ // ## `event_id` Assignment
/* FP:profiling.rs-0040 */ //
/* FP:profiling.rs-0041 */ // As far as `measureme` is concerned, `event_id`s are just strings. However,
/* FP:profiling.rs-0042 */ // it would incur too much overhead to generate and persist each `event_id`
/* FP:profiling.rs-0043 */ // string at the point where the event is recorded. In order to make this more
/* FP:profiling.rs-0044 */ // efficient `measureme` has two features:
/* FP:profiling.rs-0045 */ //
/* FP:profiling.rs-0046 */ // - Strings can share their content, so that re-occurring parts don't have to
/* FP:profiling.rs-0047 */ //   be copied over and over again. One allocates a string in `measureme` and
/* FP:profiling.rs-0048 */ //   gets back a `StringId`. This `StringId` is then used to refer to that
/* FP:profiling.rs-0049 */ //   string. `measureme` strings are actually DAGs of string components so that
/* FP:profiling.rs-0050 */ //   arbitrary sharing of substrings can be done efficiently. This is useful
/* FP:profiling.rs-0051 */ //   because `event_id`s contain lots of redundant text like query names or
/* FP:profiling.rs-0052 */ //   def-path components.
/* FP:profiling.rs-0053 */ //
/* FP:profiling.rs-0054 */ // - `StringId`s can be "virtual" which means that the client picks a numeric
/* FP:profiling.rs-0055 */ //   ID according to some application-specific scheme and can later make that
/* FP:profiling.rs-0056 */ //   ID be mapped to an actual string. This is used to cheaply generate
/* FP:profiling.rs-0057 */ //   `event_id`s while the events actually occur, causing little timing
/* FP:profiling.rs-0058 */ //   distortion, and then later map those `StringId`s, in bulk, to actual
/* FP:profiling.rs-0059 */ //   `event_id` strings. This way the largest part of the tracing overhead is
/* FP:profiling.rs-0060 */ //   localized to one contiguous chunk of time.
/* FP:profiling.rs-0061 */ //
/* FP:profiling.rs-0062 */ // How are these `event_id`s generated in the compiler? For things that occur
/* FP:profiling.rs-0063 */ // infrequently (e.g. "generic activities"), we just allocate the string the
/* FP:profiling.rs-0064 */ // first time it is used and then keep the `StringId` in a hash table. This
/* FP:profiling.rs-0065 */ // is implemented in `SelfProfiler::get_or_alloc_cached_string()`.
/* FP:profiling.rs-0066 */ //
/* FP:profiling.rs-0067 */ // For queries it gets more interesting: First we need a unique numeric ID for
/* FP:profiling.rs-0068 */ // each query invocation (the `QueryInvocationId`). This ID is used as the
/* FP:profiling.rs-0069 */ // virtual `StringId` we use as `event_id` for a given event. This ID has to
/* FP:profiling.rs-0070 */ // be available both when the query is executed and later, together with the
/* FP:profiling.rs-0071 */ // query key, when we allocate the actual `event_id` strings in bulk.
/* FP:profiling.rs-0072 */ //
/* FP:profiling.rs-0073 */ // We could make the compiler generate and keep track of such an ID for each
/* FP:profiling.rs-0074 */ // query invocation but luckily we already have something that fits all the
/* FP:profiling.rs-0075 */ // the requirements: the query's `DepNodeIndex`. So we use the numeric value
/* FP:profiling.rs-0076 */ // of the `DepNodeIndex` as `event_id` when recording the event and then,
/* FP:profiling.rs-0077 */ // just before the query context is dropped, we walk the entire query cache
/* FP:profiling.rs-0078 */ // (which stores the `DepNodeIndex` along with the query key for each
/* FP:profiling.rs-0079 */ // invocation) and allocate the corresponding strings together with a mapping
/* FP:profiling.rs-0080 */ // for `DepNodeIndex as StringId`.
/* FP:profiling.rs-0081 */ //
/* FP:profiling.rs-0082 */ // [mm]: https://github.com/rust-lang/measureme/
/* FP:profiling.rs-0083 */ 
/* FP:profiling.rs-0084 */ use std::borrow::Borrow;
/* FP:profiling.rs-0085 */ use std::collections::hash_map::Entry;
/* FP:profiling.rs-0086 */ use std::error::Error;
/* FP:profiling.rs-0087 */ use std::fmt::Display;
/* FP:profiling.rs-0088 */ use std::intrinsics::unlikely;
/* FP:profiling.rs-0089 */ use std::path::Path;
/* FP:profiling.rs-0090 */ use std::sync::Arc;
/* FP:profiling.rs-0091 */ use std::sync::atomic::Ordering;
/* FP:profiling.rs-0092 */ use std::time::{Duration, Instant};
/* FP:profiling.rs-0093 */ use std::{fs, process};
/* FP:profiling.rs-0094 */ 
/* FP:profiling.rs-0095 */ pub use measureme::EventId;
/* FP:profiling.rs-0096 */ use measureme::{EventIdBuilder, Profiler, SerializableString, StringId};
/* FP:profiling.rs-0097 */ use parking_lot::RwLock;
/* FP:profiling.rs-0098 */ use smallvec::SmallVec;
/* FP:profiling.rs-0099 */ use tracing::warn;
/* FP:profiling.rs-0100 */ 
/* FP:profiling.rs-0101 */ use crate::fx::FxHashMap;
/* FP:profiling.rs-0102 */ use crate::outline;
/* FP:profiling.rs-0103 */ use crate::sync::AtomicU64;
/* FP:profiling.rs-0104 */ 
/* FP:profiling.rs-0105 */ bitflags::bitflags! {
/* FP:profiling.rs-0106 */     #[derive(Clone, Copy)]
/* FP:profiling.rs-0107 */     struct EventFilter: u16 {
/* FP:profiling.rs-0108 */         const GENERIC_ACTIVITIES  = 1 << 0;
/* FP:profiling.rs-0109 */         const QUERY_PROVIDERS     = 1 << 1;
/* FP:profiling.rs-0110 */         /// Store detailed instant events, including timestamp and thread ID,
/* FP:profiling.rs-0111 */         /// per each query cache hit. Note that this is quite expensive.
/* FP:profiling.rs-0112 */         const QUERY_CACHE_HITS    = 1 << 2;
/* FP:profiling.rs-0113 */         const QUERY_BLOCKED       = 1 << 3;
/* FP:profiling.rs-0114 */         const INCR_CACHE_LOADS    = 1 << 4;
/* FP:profiling.rs-0115 */ 
/* FP:profiling.rs-0116 */         const QUERY_KEYS          = 1 << 5;
/* FP:profiling.rs-0117 */         const FUNCTION_ARGS       = 1 << 6;
/* FP:profiling.rs-0118 */         const LLVM                = 1 << 7;
/* FP:profiling.rs-0119 */         const INCR_RESULT_HASHING = 1 << 8;
/* FP:profiling.rs-0120 */         const ARTIFACT_SIZES      = 1 << 9;
/* FP:profiling.rs-0121 */         /// Store aggregated counts of cache hits per query invocation.
/* FP:profiling.rs-0122 */         const QUERY_CACHE_HIT_COUNTS  = 1 << 10;
/* FP:profiling.rs-0123 */ 
/* FP:profiling.rs-0124 */         const DEFAULT = Self::GENERIC_ACTIVITIES.bits() |
/* FP:profiling.rs-0125 */                         Self::QUERY_PROVIDERS.bits() |
/* FP:profiling.rs-0126 */                         Self::QUERY_BLOCKED.bits() |
/* FP:profiling.rs-0127 */                         Self::INCR_CACHE_LOADS.bits() |
/* FP:profiling.rs-0128 */                         Self::INCR_RESULT_HASHING.bits() |
/* FP:profiling.rs-0129 */                         Self::ARTIFACT_SIZES.bits() |
/* FP:profiling.rs-0130 */                         Self::QUERY_CACHE_HIT_COUNTS.bits();
/* FP:profiling.rs-0131 */ 
/* FP:profiling.rs-0132 */         const ARGS = Self::QUERY_KEYS.bits() | Self::FUNCTION_ARGS.bits();
/* FP:profiling.rs-0133 */         const QUERY_CACHE_HIT_COMBINED = Self::QUERY_CACHE_HITS.bits() | Self::QUERY_CACHE_HIT_COUNTS.bits();
/* FP:profiling.rs-0134 */     }
/* FP:profiling.rs-0135 */ }
/* FP:profiling.rs-0136 */ 
/* FP:profiling.rs-0137 */ // keep this in sync with the `-Z self-profile-events` help message in rustc_session/options.rs
/* FP:profiling.rs-0138 */ const EVENT_FILTERS_BY_NAME: &[(&str, EventFilter)] = &[
/* FP:profiling.rs-0139 */     ("none", EventFilter::empty()),
/* FP:profiling.rs-0140 */     ("all", EventFilter::all()),
/* FP:profiling.rs-0141 */     ("default", EventFilter::DEFAULT),
/* FP:profiling.rs-0142 */     ("generic-activity", EventFilter::GENERIC_ACTIVITIES),
/* FP:profiling.rs-0143 */     ("query-provider", EventFilter::QUERY_PROVIDERS),
/* FP:profiling.rs-0144 */     ("query-cache-hit", EventFilter::QUERY_CACHE_HITS),
/* FP:profiling.rs-0145 */     ("query-cache-hit-count", EventFilter::QUERY_CACHE_HIT_COUNTS),
/* FP:profiling.rs-0146 */     ("query-blocked", EventFilter::QUERY_BLOCKED),
/* FP:profiling.rs-0147 */     ("incr-cache-load", EventFilter::INCR_CACHE_LOADS),
/* FP:profiling.rs-0148 */     ("query-keys", EventFilter::QUERY_KEYS),
/* FP:profiling.rs-0149 */     ("function-args", EventFilter::FUNCTION_ARGS),
/* FP:profiling.rs-0150 */     ("args", EventFilter::ARGS),
/* FP:profiling.rs-0151 */     ("llvm", EventFilter::LLVM),
/* FP:profiling.rs-0152 */     ("incr-result-hashing", EventFilter::INCR_RESULT_HASHING),
/* FP:profiling.rs-0153 */     ("artifact-sizes", EventFilter::ARTIFACT_SIZES),
/* FP:profiling.rs-0154 */ ];
/* FP:profiling.rs-0155 */ 
/* FP:profiling.rs-0156 */ /// Something that uniquely identifies a query invocation.
/* FP:profiling.rs-0157 */ pub struct QueryInvocationId(pub u32);
/* FP:profiling.rs-0158 */ 
/* FP:profiling.rs-0159 */ /// Which format to use for `-Z time-passes`
/* FP:profiling.rs-0160 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:profiling.rs-0161 */ pub enum TimePassesFormat {
/* FP:profiling.rs-0162 */     /// Emit human readable text
/* FP:profiling.rs-0163 */     Text,
/* FP:profiling.rs-0164 */     /// Emit structured JSON
/* FP:profiling.rs-0165 */     Json,
/* FP:profiling.rs-0166 */ }
/* FP:profiling.rs-0167 */ 
/* FP:profiling.rs-0168 */ /// A reference to the SelfProfiler. It can be cloned and sent across thread
/* FP:profiling.rs-0169 */ /// boundaries at will.
/* FP:profiling.rs-0170 */ #[derive(Clone)]
/* FP:profiling.rs-0171 */ pub struct SelfProfilerRef {
/* FP:profiling.rs-0172 */     // This field is `None` if self-profiling is disabled for the current
/* FP:profiling.rs-0173 */     // compilation session.
/* FP:profiling.rs-0174 */     profiler: Option<Arc<SelfProfiler>>,
/* FP:profiling.rs-0175 */ 
/* FP:profiling.rs-0176 */     // We store the filter mask directly in the reference because that doesn't
/* FP:profiling.rs-0177 */     // cost anything and allows for filtering with checking if the profiler is
/* FP:profiling.rs-0178 */     // actually enabled.
/* FP:profiling.rs-0179 */     event_filter_mask: EventFilter,
/* FP:profiling.rs-0180 */ 
/* FP:profiling.rs-0181 */     // Print verbose generic activities to stderr.
/* FP:profiling.rs-0182 */     print_verbose_generic_activities: Option<TimePassesFormat>,
/* FP:profiling.rs-0183 */ }
/* FP:profiling.rs-0184 */ 
/* FP:profiling.rs-0185 */ impl SelfProfilerRef {
/* FP:profiling.rs-0186 */     pub fn new(
/* FP:profiling.rs-0187 */         profiler: Option<Arc<SelfProfiler>>,
/* FP:profiling.rs-0188 */         print_verbose_generic_activities: Option<TimePassesFormat>,
/* FP:profiling.rs-0189 */     ) -> SelfProfilerRef {
/* FP:profiling.rs-0190 */         // If there is no SelfProfiler then the filter mask is set to NONE,
/* FP:profiling.rs-0191 */         // ensuring that nothing ever tries to actually access it.
/* FP:profiling.rs-0192 */         let event_filter_mask =
/* FP:profiling.rs-0193 */             profiler.as_ref().map_or(EventFilter::empty(), |p| p.event_filter_mask);
/* FP:profiling.rs-0194 */ 
/* FP:profiling.rs-0195 */         SelfProfilerRef { profiler, event_filter_mask, print_verbose_generic_activities }
/* FP:profiling.rs-0196 */     }
/* FP:profiling.rs-0197 */ 
/* FP:profiling.rs-0198 */     /// This shim makes sure that calls only get executed if the filter mask
/* FP:profiling.rs-0199 */     /// lets them pass. It also contains some trickery to make sure that
/* FP:profiling.rs-0200 */     /// code is optimized for non-profiling compilation sessions, i.e. anything
/* FP:profiling.rs-0201 */     /// past the filter check is never inlined so it doesn't clutter the fast
/* FP:profiling.rs-0202 */     /// path.
/* FP:profiling.rs-0203 */     #[inline(always)]
/* FP:profiling.rs-0204 */     fn exec<F>(&self, event_filter: EventFilter, f: F) -> TimingGuard<'_>
/* FP:profiling.rs-0205 */     where
/* FP:profiling.rs-0206 */         F: for<'a> FnOnce(&'a SelfProfiler) -> TimingGuard<'a>,
/* FP:profiling.rs-0207 */     {
/* FP:profiling.rs-0208 */         #[inline(never)]
/* FP:profiling.rs-0209 */         #[cold]
/* FP:profiling.rs-0210 */         fn cold_call<F>(profiler_ref: &SelfProfilerRef, f: F) -> TimingGuard<'_>
/* FP:profiling.rs-0211 */         where
/* FP:profiling.rs-0212 */             F: for<'a> FnOnce(&'a SelfProfiler) -> TimingGuard<'a>,
/* FP:profiling.rs-0213 */         {
/* FP:profiling.rs-0214 */             let profiler = profiler_ref.profiler.as_ref().unwrap();
/* FP:profiling.rs-0215 */             f(profiler)
/* FP:profiling.rs-0216 */         }
/* FP:profiling.rs-0217 */ 
/* FP:profiling.rs-0218 */         if self.event_filter_mask.contains(event_filter) {
/* FP:profiling.rs-0219 */             cold_call(self, f)
/* FP:profiling.rs-0220 */         } else {
/* FP:profiling.rs-0221 */             TimingGuard::none()
/* FP:profiling.rs-0222 */         }
/* FP:profiling.rs-0223 */     }
/* FP:profiling.rs-0224 */ 
/* FP:profiling.rs-0225 */     /// Start profiling a verbose generic activity. Profiling continues until the
/* FP:profiling.rs-0226 */     /// VerboseTimingGuard returned from this call is dropped. In addition to recording
/* FP:profiling.rs-0227 */     /// a measureme event, "verbose" generic activities also print a timing entry to
/* FP:profiling.rs-0228 */     /// stderr if the compiler is invoked with -Ztime-passes.
/* FP:profiling.rs-0229 */     pub fn verbose_generic_activity(&self, event_label: &'static str) -> VerboseTimingGuard<'_> {
/* FP:profiling.rs-0230 */         let message_and_format =
/* FP:profiling.rs-0231 */             self.print_verbose_generic_activities.map(|format| (event_label.to_owned(), format));
/* FP:profiling.rs-0232 */ 
/* FP:profiling.rs-0233 */         VerboseTimingGuard::start(message_and_format, self.generic_activity(event_label))
/* FP:profiling.rs-0234 */     }
/* FP:profiling.rs-0235 */ 
/* FP:profiling.rs-0236 */     /// Like `verbose_generic_activity`, but with an extra arg.
/* FP:profiling.rs-0237 */     pub fn verbose_generic_activity_with_arg<A>(
/* FP:profiling.rs-0238 */         &self,
/* FP:profiling.rs-0239 */         event_label: &'static str,
/* FP:profiling.rs-0240 */         event_arg: A,
/* FP:profiling.rs-0241 */     ) -> VerboseTimingGuard<'_>
/* FP:profiling.rs-0242 */     where
/* FP:profiling.rs-0243 */         A: Borrow<str> + Into<String>,
/* FP:profiling.rs-0244 */     {
/* FP:profiling.rs-0245 */         let message_and_format = self
/* FP:profiling.rs-0246 */             .print_verbose_generic_activities
/* FP:profiling.rs-0247 */             .map(|format| (format!("{}({})", event_label, event_arg.borrow()), format));
/* FP:profiling.rs-0248 */ 
/* FP:profiling.rs-0249 */         VerboseTimingGuard::start(
/* FP:profiling.rs-0250 */             message_and_format,
/* FP:profiling.rs-0251 */             self.generic_activity_with_arg(event_label, event_arg),
/* FP:profiling.rs-0252 */         )
/* FP:profiling.rs-0253 */     }
/* FP:profiling.rs-0254 */ 
/* FP:profiling.rs-0255 */     /// Start profiling a generic activity. Profiling continues until the
/* FP:profiling.rs-0256 */     /// TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0257 */     #[inline(always)]
/* FP:profiling.rs-0258 */     pub fn generic_activity(&self, event_label: &'static str) -> TimingGuard<'_> {
/* FP:profiling.rs-0259 */         self.exec(EventFilter::GENERIC_ACTIVITIES, |profiler| {
/* FP:profiling.rs-0260 */             let event_label = profiler.get_or_alloc_cached_string(event_label);
/* FP:profiling.rs-0261 */             let event_id = EventId::from_label(event_label);
/* FP:profiling.rs-0262 */             TimingGuard::start(profiler, profiler.generic_activity_event_kind, event_id)
/* FP:profiling.rs-0263 */         })
/* FP:profiling.rs-0264 */     }
/* FP:profiling.rs-0265 */ 
/* FP:profiling.rs-0266 */     /// Start profiling with some event filter for a given event. Profiling continues until the
/* FP:profiling.rs-0267 */     /// TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0268 */     #[inline(always)]
/* FP:profiling.rs-0269 */     pub fn generic_activity_with_event_id(&self, event_id: EventId) -> TimingGuard<'_> {
/* FP:profiling.rs-0270 */         self.exec(EventFilter::GENERIC_ACTIVITIES, |profiler| {
/* FP:profiling.rs-0271 */             TimingGuard::start(profiler, profiler.generic_activity_event_kind, event_id)
/* FP:profiling.rs-0272 */         })
/* FP:profiling.rs-0273 */     }
/* FP:profiling.rs-0274 */ 
/* FP:profiling.rs-0275 */     /// Start profiling a generic activity. Profiling continues until the
/* FP:profiling.rs-0276 */     /// TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0277 */     #[inline(always)]
/* FP:profiling.rs-0278 */     pub fn generic_activity_with_arg<A>(
/* FP:profiling.rs-0279 */         &self,
/* FP:profiling.rs-0280 */         event_label: &'static str,
/* FP:profiling.rs-0281 */         event_arg: A,
/* FP:profiling.rs-0282 */     ) -> TimingGuard<'_>
/* FP:profiling.rs-0283 */     where
/* FP:profiling.rs-0284 */         A: Borrow<str> + Into<String>,
/* FP:profiling.rs-0285 */     {
/* FP:profiling.rs-0286 */         self.exec(EventFilter::GENERIC_ACTIVITIES, |profiler| {
/* FP:profiling.rs-0287 */             let builder = EventIdBuilder::new(&profiler.profiler);
/* FP:profiling.rs-0288 */             let event_label = profiler.get_or_alloc_cached_string(event_label);
/* FP:profiling.rs-0289 */             let event_id = if profiler.event_filter_mask.contains(EventFilter::FUNCTION_ARGS) {
/* FP:profiling.rs-0290 */                 let event_arg = profiler.get_or_alloc_cached_string(event_arg);
/* FP:profiling.rs-0291 */                 builder.from_label_and_arg(event_label, event_arg)
/* FP:profiling.rs-0292 */             } else {
/* FP:profiling.rs-0293 */                 builder.from_label(event_label)
/* FP:profiling.rs-0294 */             };
/* FP:profiling.rs-0295 */             TimingGuard::start(profiler, profiler.generic_activity_event_kind, event_id)
/* FP:profiling.rs-0296 */         })
/* FP:profiling.rs-0297 */     }
/* FP:profiling.rs-0298 */ 
/* FP:profiling.rs-0299 */     /// Start profiling a generic activity, allowing costly arguments to be recorded. Profiling
/* FP:profiling.rs-0300 */     /// continues until the `TimingGuard` returned from this call is dropped.
/* FP:profiling.rs-0301 */     ///
/* FP:profiling.rs-0302 */     /// If the arguments to a generic activity are cheap to create, use `generic_activity_with_arg`
/* FP:profiling.rs-0303 */     /// or `generic_activity_with_args` for their simpler API. However, if they are costly or
/* FP:profiling.rs-0304 */     /// require allocation in sufficiently hot contexts, then this allows for a closure to be called
/* FP:profiling.rs-0305 */     /// only when arguments were asked to be recorded via `-Z self-profile-events=args`.
/* FP:profiling.rs-0306 */     ///
/* FP:profiling.rs-0307 */     /// In this case, the closure will be passed a `&mut EventArgRecorder`, to help with recording
/* FP:profiling.rs-0308 */     /// one or many arguments within the generic activity being profiled, by calling its
/* FP:profiling.rs-0309 */     /// `record_arg` method for example.
/* FP:profiling.rs-0310 */     ///
/* FP:profiling.rs-0311 */     /// This `EventArgRecorder` may implement more specific traits from other rustc crates, e.g. for
/* FP:profiling.rs-0312 */     /// richer handling of rustc-specific argument types, while keeping this single entry-point API
/* FP:profiling.rs-0313 */     /// for recording arguments.
/* FP:profiling.rs-0314 */     ///
/* FP:profiling.rs-0315 */     /// Note: recording at least one argument is *required* for the self-profiler to create the
/* FP:profiling.rs-0316 */     /// `TimingGuard`. A panic will be triggered if that doesn't happen. This function exists
/* FP:profiling.rs-0317 */     /// explicitly to record arguments, so it fails loudly when there are none to record.
/* FP:profiling.rs-0318 */     ///
/* FP:profiling.rs-0319 */     #[inline(always)]
/* FP:profiling.rs-0320 */     pub fn generic_activity_with_arg_recorder<F>(
/* FP:profiling.rs-0321 */         &self,
/* FP:profiling.rs-0322 */         event_label: &'static str,
/* FP:profiling.rs-0323 */         mut f: F,
/* FP:profiling.rs-0324 */     ) -> TimingGuard<'_>
/* FP:profiling.rs-0325 */     where
/* FP:profiling.rs-0326 */         F: FnMut(&mut EventArgRecorder<'_>),
/* FP:profiling.rs-0327 */     {
/* FP:profiling.rs-0328 */         // Ensure this event will only be recorded when self-profiling is turned on.
/* FP:profiling.rs-0329 */         self.exec(EventFilter::GENERIC_ACTIVITIES, |profiler| {
/* FP:profiling.rs-0330 */             let builder = EventIdBuilder::new(&profiler.profiler);
/* FP:profiling.rs-0331 */             let event_label = profiler.get_or_alloc_cached_string(event_label);
/* FP:profiling.rs-0332 */ 
/* FP:profiling.rs-0333 */             // Ensure the closure to create event arguments will only be called when argument
/* FP:profiling.rs-0334 */             // recording is turned on.
/* FP:profiling.rs-0335 */             let event_id = if profiler.event_filter_mask.contains(EventFilter::FUNCTION_ARGS) {
/* FP:profiling.rs-0336 */                 // Set up the builder and call the user-provided closure to record potentially
/* FP:profiling.rs-0337 */                 // costly event arguments.
/* FP:profiling.rs-0338 */                 let mut recorder = EventArgRecorder { profiler, args: SmallVec::new() };
/* FP:profiling.rs-0339 */                 f(&mut recorder);
/* FP:profiling.rs-0340 */ 
/* FP:profiling.rs-0341 */                 // It is expected that the closure will record at least one argument. If that
/* FP:profiling.rs-0342 */                 // doesn't happen, it's a bug: we've been explicitly called in order to record
/* FP:profiling.rs-0343 */                 // arguments, so we fail loudly when there are none to record.
/* FP:profiling.rs-0344 */                 if recorder.args.is_empty() {
/* FP:profiling.rs-0345 */                     panic!(
/* FP:profiling.rs-0346 */                         "The closure passed to `generic_activity_with_arg_recorder` needs to \
/* FP:profiling.rs-0347 */                          record at least one argument"
/* FP:profiling.rs-0348 */                     );
/* FP:profiling.rs-0349 */                 }
/* FP:profiling.rs-0350 */ 
/* FP:profiling.rs-0351 */                 builder.from_label_and_args(event_label, &recorder.args)
/* FP:profiling.rs-0352 */             } else {
/* FP:profiling.rs-0353 */                 builder.from_label(event_label)
/* FP:profiling.rs-0354 */             };
/* FP:profiling.rs-0355 */             TimingGuard::start(profiler, profiler.generic_activity_event_kind, event_id)
/* FP:profiling.rs-0356 */         })
/* FP:profiling.rs-0357 */     }
/* FP:profiling.rs-0358 */ 
/* FP:profiling.rs-0359 */     /// Record the size of an artifact that the compiler produces
/* FP:profiling.rs-0360 */     ///
/* FP:profiling.rs-0361 */     /// `artifact_kind` is the class of artifact (e.g., query_cache, object_file, etc.)
/* FP:profiling.rs-0362 */     /// `artifact_name` is an identifier to the specific artifact being stored (usually a filename)
/* FP:profiling.rs-0363 */     #[inline(always)]
/* FP:profiling.rs-0364 */     pub fn artifact_size<A>(&self, artifact_kind: &str, artifact_name: A, size: u64)
/* FP:profiling.rs-0365 */     where
/* FP:profiling.rs-0366 */         A: Borrow<str> + Into<String>,
/* FP:profiling.rs-0367 */     {
/* FP:profiling.rs-0368 */         drop(self.exec(EventFilter::ARTIFACT_SIZES, |profiler| {
/* FP:profiling.rs-0369 */             let builder = EventIdBuilder::new(&profiler.profiler);
/* FP:profiling.rs-0370 */             let event_label = profiler.get_or_alloc_cached_string(artifact_kind);
/* FP:profiling.rs-0371 */             let event_arg = profiler.get_or_alloc_cached_string(artifact_name);
/* FP:profiling.rs-0372 */             let event_id = builder.from_label_and_arg(event_label, event_arg);
/* FP:profiling.rs-0373 */             let thread_id = get_thread_id();
/* FP:profiling.rs-0374 */ 
/* FP:profiling.rs-0375 */             profiler.profiler.record_integer_event(
/* FP:profiling.rs-0376 */                 profiler.artifact_size_event_kind,
/* FP:profiling.rs-0377 */                 event_id,
/* FP:profiling.rs-0378 */                 thread_id,
/* FP:profiling.rs-0379 */                 size,
/* FP:profiling.rs-0380 */             );
/* FP:profiling.rs-0381 */ 
/* FP:profiling.rs-0382 */             TimingGuard::none()
/* FP:profiling.rs-0383 */         }))
/* FP:profiling.rs-0384 */     }
/* FP:profiling.rs-0385 */ 
/* FP:profiling.rs-0386 */     #[inline(always)]
/* FP:profiling.rs-0387 */     pub fn generic_activity_with_args(
/* FP:profiling.rs-0388 */         &self,
/* FP:profiling.rs-0389 */         event_label: &'static str,
/* FP:profiling.rs-0390 */         event_args: &[String],
/* FP:profiling.rs-0391 */     ) -> TimingGuard<'_> {
/* FP:profiling.rs-0392 */         self.exec(EventFilter::GENERIC_ACTIVITIES, |profiler| {
/* FP:profiling.rs-0393 */             let builder = EventIdBuilder::new(&profiler.profiler);
/* FP:profiling.rs-0394 */             let event_label = profiler.get_or_alloc_cached_string(event_label);
/* FP:profiling.rs-0395 */             let event_id = if profiler.event_filter_mask.contains(EventFilter::FUNCTION_ARGS) {
/* FP:profiling.rs-0396 */                 let event_args: Vec<_> = event_args
/* FP:profiling.rs-0397 */                     .iter()
/* FP:profiling.rs-0398 */                     .map(|s| profiler.get_or_alloc_cached_string(&s[..]))
/* FP:profiling.rs-0399 */                     .collect();
/* FP:profiling.rs-0400 */                 builder.from_label_and_args(event_label, &event_args)
/* FP:profiling.rs-0401 */             } else {
/* FP:profiling.rs-0402 */                 builder.from_label(event_label)
/* FP:profiling.rs-0403 */             };
/* FP:profiling.rs-0404 */             TimingGuard::start(profiler, profiler.generic_activity_event_kind, event_id)
/* FP:profiling.rs-0405 */         })
/* FP:profiling.rs-0406 */     }
/* FP:profiling.rs-0407 */ 
/* FP:profiling.rs-0408 */     /// Start profiling a query provider. Profiling continues until the
/* FP:profiling.rs-0409 */     /// TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0410 */     #[inline(always)]
/* FP:profiling.rs-0411 */     pub fn query_provider(&self) -> TimingGuard<'_> {
/* FP:profiling.rs-0412 */         self.exec(EventFilter::QUERY_PROVIDERS, |profiler| {
/* FP:profiling.rs-0413 */             TimingGuard::start(profiler, profiler.query_event_kind, EventId::INVALID)
/* FP:profiling.rs-0414 */         })
/* FP:profiling.rs-0415 */     }
/* FP:profiling.rs-0416 */ 
/* FP:profiling.rs-0417 */     /// Record a query in-memory cache hit.
/* FP:profiling.rs-0418 */     #[inline(always)]
/* FP:profiling.rs-0419 */     pub fn query_cache_hit(&self, query_invocation_id: QueryInvocationId) {
/* FP:profiling.rs-0420 */         #[inline(never)]
/* FP:profiling.rs-0421 */         #[cold]
/* FP:profiling.rs-0422 */         fn cold_call(profiler_ref: &SelfProfilerRef, query_invocation_id: QueryInvocationId) {
/* FP:profiling.rs-0423 */             if profiler_ref.event_filter_mask.contains(EventFilter::QUERY_CACHE_HIT_COUNTS) {
/* FP:profiling.rs-0424 */                 profiler_ref
/* FP:profiling.rs-0425 */                     .profiler
/* FP:profiling.rs-0426 */                     .as_ref()
/* FP:profiling.rs-0427 */                     .unwrap()
/* FP:profiling.rs-0428 */                     .increment_query_cache_hit_counters(QueryInvocationId(query_invocation_id.0));
/* FP:profiling.rs-0429 */             }
/* FP:profiling.rs-0430 */             if unlikely(profiler_ref.event_filter_mask.contains(EventFilter::QUERY_CACHE_HITS)) {
/* FP:profiling.rs-0431 */                 profiler_ref.instant_query_event(
/* FP:profiling.rs-0432 */                     |profiler| profiler.query_cache_hit_event_kind,
/* FP:profiling.rs-0433 */                     query_invocation_id,
/* FP:profiling.rs-0434 */                 );
/* FP:profiling.rs-0435 */             }
/* FP:profiling.rs-0436 */         }
/* FP:profiling.rs-0437 */ 
/* FP:profiling.rs-0438 */         // We check both kinds of query cache hit events at once, to reduce overhead in the
/* FP:profiling.rs-0439 */         // common case (with self-profile disabled).
/* FP:profiling.rs-0440 */         if unlikely(self.event_filter_mask.intersects(EventFilter::QUERY_CACHE_HIT_COMBINED)) {
/* FP:profiling.rs-0441 */             cold_call(self, query_invocation_id);
/* FP:profiling.rs-0442 */         }
/* FP:profiling.rs-0443 */     }
/* FP:profiling.rs-0444 */ 
/* FP:profiling.rs-0445 */     /// Start profiling a query being blocked on a concurrent execution.
/* FP:profiling.rs-0446 */     /// Profiling continues until the TimingGuard returned from this call is
/* FP:profiling.rs-0447 */     /// dropped.
/* FP:profiling.rs-0448 */     #[inline(always)]
/* FP:profiling.rs-0449 */     pub fn query_blocked(&self) -> TimingGuard<'_> {
/* FP:profiling.rs-0450 */         self.exec(EventFilter::QUERY_BLOCKED, |profiler| {
/* FP:profiling.rs-0451 */             TimingGuard::start(profiler, profiler.query_blocked_event_kind, EventId::INVALID)
/* FP:profiling.rs-0452 */         })
/* FP:profiling.rs-0453 */     }
/* FP:profiling.rs-0454 */ 
/* FP:profiling.rs-0455 */     /// Start profiling how long it takes to load a query result from the
/* FP:profiling.rs-0456 */     /// incremental compilation on-disk cache. Profiling continues until the
/* FP:profiling.rs-0457 */     /// TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0458 */     #[inline(always)]
/* FP:profiling.rs-0459 */     pub fn incr_cache_loading(&self) -> TimingGuard<'_> {
/* FP:profiling.rs-0460 */         self.exec(EventFilter::INCR_CACHE_LOADS, |profiler| {
/* FP:profiling.rs-0461 */             TimingGuard::start(
/* FP:profiling.rs-0462 */                 profiler,
/* FP:profiling.rs-0463 */                 profiler.incremental_load_result_event_kind,
/* FP:profiling.rs-0464 */                 EventId::INVALID,
/* FP:profiling.rs-0465 */             )
/* FP:profiling.rs-0466 */         })
/* FP:profiling.rs-0467 */     }
/* FP:profiling.rs-0468 */ 
/* FP:profiling.rs-0469 */     /// Start profiling how long it takes to hash query results for incremental compilation.
/* FP:profiling.rs-0470 */     /// Profiling continues until the TimingGuard returned from this call is dropped.
/* FP:profiling.rs-0471 */     #[inline(always)]
/* FP:profiling.rs-0472 */     pub fn incr_result_hashing(&self) -> TimingGuard<'_> {
/* FP:profiling.rs-0473 */         self.exec(EventFilter::INCR_RESULT_HASHING, |profiler| {
/* FP:profiling.rs-0474 */             TimingGuard::start(
/* FP:profiling.rs-0475 */                 profiler,
/* FP:profiling.rs-0476 */                 profiler.incremental_result_hashing_event_kind,
/* FP:profiling.rs-0477 */                 EventId::INVALID,
/* FP:profiling.rs-0478 */             )
/* FP:profiling.rs-0479 */         })
/* FP:profiling.rs-0480 */     }
/* FP:profiling.rs-0481 */ 
/* FP:profiling.rs-0482 */     #[inline(always)]
/* FP:profiling.rs-0483 */     fn instant_query_event(
/* FP:profiling.rs-0484 */         &self,
/* FP:profiling.rs-0485 */         event_kind: fn(&SelfProfiler) -> StringId,
/* FP:profiling.rs-0486 */         query_invocation_id: QueryInvocationId,
/* FP:profiling.rs-0487 */     ) {
/* FP:profiling.rs-0488 */         let event_id = StringId::new_virtual(query_invocation_id.0);
/* FP:profiling.rs-0489 */         let thread_id = get_thread_id();
/* FP:profiling.rs-0490 */         let profiler = self.profiler.as_ref().unwrap();
/* FP:profiling.rs-0491 */         profiler.profiler.record_instant_event(
/* FP:profiling.rs-0492 */             event_kind(profiler),
/* FP:profiling.rs-0493 */             EventId::from_virtual(event_id),
/* FP:profiling.rs-0494 */             thread_id,
/* FP:profiling.rs-0495 */         );
/* FP:profiling.rs-0496 */     }
/* FP:profiling.rs-0497 */ 
/* FP:profiling.rs-0498 */     pub fn with_profiler(&self, f: impl FnOnce(&SelfProfiler)) {
/* FP:profiling.rs-0499 */         if let Some(profiler) = &self.profiler {
/* FP:profiling.rs-0500 */             f(profiler)
/* FP:profiling.rs-0501 */         }
/* FP:profiling.rs-0502 */     }
/* FP:profiling.rs-0503 */ 
/* FP:profiling.rs-0504 */     /// Gets a `StringId` for the given string. This method makes sure that
/* FP:profiling.rs-0505 */     /// any strings going through it will only be allocated once in the
/* FP:profiling.rs-0506 */     /// profiling data.
/* FP:profiling.rs-0507 */     /// Returns `None` if the self-profiling is not enabled.
/* FP:profiling.rs-0508 */     pub fn get_or_alloc_cached_string(&self, s: &str) -> Option<StringId> {
/* FP:profiling.rs-0509 */         self.profiler.as_ref().map(|p| p.get_or_alloc_cached_string(s))
/* FP:profiling.rs-0510 */     }
/* FP:profiling.rs-0511 */ 
/* FP:profiling.rs-0512 */     /// Store query cache hits to the self-profile log.
/* FP:profiling.rs-0513 */     /// Should be called once at the end of the compilation session.
/* FP:profiling.rs-0514 */     ///
/* FP:profiling.rs-0515 */     /// The cache hits are stored per **query invocation**, not **per query kind/type**.
/* FP:profiling.rs-0516 */     /// `analyzeme` can later deduplicate individual query labels from the QueryInvocationId event
/* FP:profiling.rs-0517 */     /// IDs.
/* FP:profiling.rs-0518 */     pub fn store_query_cache_hits(&self) {
/* FP:profiling.rs-0519 */         if self.event_filter_mask.contains(EventFilter::QUERY_CACHE_HIT_COUNTS) {
/* FP:profiling.rs-0520 */             let profiler = self.profiler.as_ref().unwrap();
/* FP:profiling.rs-0521 */             let query_hits = profiler.query_hits.read();
/* FP:profiling.rs-0522 */             let builder = EventIdBuilder::new(&profiler.profiler);
/* FP:profiling.rs-0523 */             let thread_id = get_thread_id();
/* FP:profiling.rs-0524 */             for (query_invocation, hit_count) in query_hits.iter().enumerate() {
/* FP:profiling.rs-0525 */                 let hit_count = hit_count.load(Ordering::Relaxed);
/* FP:profiling.rs-0526 */                 // No need to record empty cache hit counts
/* FP:profiling.rs-0527 */                 if hit_count > 0 {
/* FP:profiling.rs-0528 */                     let event_id =
/* FP:profiling.rs-0529 */                         builder.from_label(StringId::new_virtual(query_invocation as u64));
/* FP:profiling.rs-0530 */                     profiler.profiler.record_integer_event(
/* FP:profiling.rs-0531 */                         profiler.query_cache_hit_count_event_kind,
/* FP:profiling.rs-0532 */                         event_id,
/* FP:profiling.rs-0533 */                         thread_id,
/* FP:profiling.rs-0534 */                         hit_count,
/* FP:profiling.rs-0535 */                     );
/* FP:profiling.rs-0536 */                 }
/* FP:profiling.rs-0537 */             }
/* FP:profiling.rs-0538 */         }
/* FP:profiling.rs-0539 */     }
/* FP:profiling.rs-0540 */ 
/* FP:profiling.rs-0541 */     #[inline]
/* FP:profiling.rs-0542 */     pub fn enabled(&self) -> bool {
/* FP:profiling.rs-0543 */         self.profiler.is_some()
/* FP:profiling.rs-0544 */     }
/* FP:profiling.rs-0545 */ 
/* FP:profiling.rs-0546 */     #[inline]
/* FP:profiling.rs-0547 */     pub fn llvm_recording_enabled(&self) -> bool {
/* FP:profiling.rs-0548 */         self.event_filter_mask.contains(EventFilter::LLVM)
/* FP:profiling.rs-0549 */     }
/* FP:profiling.rs-0550 */     #[inline]
/* FP:profiling.rs-0551 */     pub fn get_self_profiler(&self) -> Option<Arc<SelfProfiler>> {
/* FP:profiling.rs-0552 */         self.profiler.clone()
/* FP:profiling.rs-0553 */     }
/* FP:profiling.rs-0554 */ 
/* FP:profiling.rs-0555 */     /// Is expensive recording of query keys and/or function arguments enabled?
/* FP:profiling.rs-0556 */     pub fn is_args_recording_enabled(&self) -> bool {
/* FP:profiling.rs-0557 */         self.enabled() && self.event_filter_mask.intersects(EventFilter::ARGS)
/* FP:profiling.rs-0558 */     }
/* FP:profiling.rs-0559 */ }
/* FP:profiling.rs-0560 */ 
/* FP:profiling.rs-0561 */ /// A helper for recording costly arguments to self-profiling events. Used with
/* FP:profiling.rs-0562 */ /// `SelfProfilerRef::generic_activity_with_arg_recorder`.
/* FP:profiling.rs-0563 */ pub struct EventArgRecorder<'p> {
/* FP:profiling.rs-0564 */     /// The `SelfProfiler` used to intern the event arguments that users will ask to record.
/* FP:profiling.rs-0565 */     profiler: &'p SelfProfiler,
/* FP:profiling.rs-0566 */ 
/* FP:profiling.rs-0567 */     /// The interned event arguments to be recorded in the generic activity event.
/* FP:profiling.rs-0568 */     ///
/* FP:profiling.rs-0569 */     /// The most common case, when actually recording event arguments, is to have one argument. Then
/* FP:profiling.rs-0570 */     /// followed by recording two, in a couple places.
/* FP:profiling.rs-0571 */     args: SmallVec<[StringId; 2]>,
/* FP:profiling.rs-0572 */ }
/* FP:profiling.rs-0573 */ 
/* FP:profiling.rs-0574 */ impl EventArgRecorder<'_> {
/* FP:profiling.rs-0575 */     /// Records a single argument within the current generic activity being profiled.
/* FP:profiling.rs-0576 */     ///
/* FP:profiling.rs-0577 */     /// Note: when self-profiling with costly event arguments, at least one argument
/* FP:profiling.rs-0578 */     /// needs to be recorded. A panic will be triggered if that doesn't happen.
/* FP:profiling.rs-0579 */     pub fn record_arg<A>(&mut self, event_arg: A)
/* FP:profiling.rs-0580 */     where
/* FP:profiling.rs-0581 */         A: Borrow<str> + Into<String>,
/* FP:profiling.rs-0582 */     {
/* FP:profiling.rs-0583 */         let event_arg = self.profiler.get_or_alloc_cached_string(event_arg);
/* FP:profiling.rs-0584 */         self.args.push(event_arg);
/* FP:profiling.rs-0585 */     }
/* FP:profiling.rs-0586 */ }
/* FP:profiling.rs-0587 */ 
/* FP:profiling.rs-0588 */ pub struct SelfProfiler {
/* FP:profiling.rs-0589 */     profiler: Profiler,
/* FP:profiling.rs-0590 */     event_filter_mask: EventFilter,
/* FP:profiling.rs-0591 */ 
/* FP:profiling.rs-0592 */     string_cache: RwLock<FxHashMap<String, StringId>>,
/* FP:profiling.rs-0593 */ 
/* FP:profiling.rs-0594 */     /// Recording individual query cache hits as "instant" measureme events
/* FP:profiling.rs-0595 */     /// is incredibly expensive. Instead of doing that, we simply aggregate
/* FP:profiling.rs-0596 */     /// cache hit *counts* per query invocation, and then store the final count
/* FP:profiling.rs-0597 */     /// of cache hits per invocation at the end of the compilation session.
/* FP:profiling.rs-0598 */     ///
/* FP:profiling.rs-0599 */     /// With this approach, we don't know the individual thread IDs and timestamps
/* FP:profiling.rs-0600 */     /// of cache hits, but it has very little overhead on top of `-Zself-profile`.
/* FP:profiling.rs-0601 */     /// Recording the cache hits as individual events made compilation 3-5x slower.
/* FP:profiling.rs-0602 */     ///
/* FP:profiling.rs-0603 */     /// Query invocation IDs should be monotonic integers, so we can store them in a vec,
/* FP:profiling.rs-0604 */     /// rather than using a hashmap.
/* FP:profiling.rs-0605 */     query_hits: RwLock<Vec<AtomicU64>>,
/* FP:profiling.rs-0606 */ 
/* FP:profiling.rs-0607 */     query_event_kind: StringId,
/* FP:profiling.rs-0608 */     generic_activity_event_kind: StringId,
/* FP:profiling.rs-0609 */     incremental_load_result_event_kind: StringId,
/* FP:profiling.rs-0610 */     incremental_result_hashing_event_kind: StringId,
/* FP:profiling.rs-0611 */     query_blocked_event_kind: StringId,
/* FP:profiling.rs-0612 */     query_cache_hit_event_kind: StringId,
/* FP:profiling.rs-0613 */     artifact_size_event_kind: StringId,
/* FP:profiling.rs-0614 */     /// Total cache hits per query invocation
/* FP:profiling.rs-0615 */     query_cache_hit_count_event_kind: StringId,
/* FP:profiling.rs-0616 */ }
/* FP:profiling.rs-0617 */ 
/* FP:profiling.rs-0618 */ impl SelfProfiler {
/* FP:profiling.rs-0619 */     pub fn new(
/* FP:profiling.rs-0620 */         output_directory: &Path,
/* FP:profiling.rs-0621 */         crate_name: Option<&str>,
/* FP:profiling.rs-0622 */         event_filters: Option<&[String]>,
/* FP:profiling.rs-0623 */         counter_name: &str,
/* FP:profiling.rs-0624 */     ) -> Result<SelfProfiler, Box<dyn Error + Send + Sync>> {
/* FP:profiling.rs-0625 */         fs::create_dir_all(output_directory)?;
/* FP:profiling.rs-0626 */ 
/* FP:profiling.rs-0627 */         let crate_name = crate_name.unwrap_or("unknown-crate");
/* FP:profiling.rs-0628 */         // HACK(eddyb) we need to pad the PID, strange as it may seem, as its
/* FP:profiling.rs-0629 */         // length can behave as a source of entropy for heap addresses, when
/* FP:profiling.rs-0630 */         // ASLR is disabled and the heap is otherwise deterministic.
/* FP:profiling.rs-0631 */         let pid: u32 = process::id();
/* FP:profiling.rs-0632 */         let filename = format!("{crate_name}-{pid:07}.rustc_profile");
/* FP:profiling.rs-0633 */         let path = output_directory.join(filename);
/* FP:profiling.rs-0634 */         let profiler =
/* FP:profiling.rs-0635 */             Profiler::with_counter(&path, measureme::counters::Counter::by_name(counter_name)?)?;
/* FP:profiling.rs-0636 */ 
/* FP:profiling.rs-0637 */         let query_event_kind = profiler.alloc_string("Query");
/* FP:profiling.rs-0638 */         let generic_activity_event_kind = profiler.alloc_string("GenericActivity");
/* FP:profiling.rs-0639 */         let incremental_load_result_event_kind = profiler.alloc_string("IncrementalLoadResult");
/* FP:profiling.rs-0640 */         let incremental_result_hashing_event_kind =
/* FP:profiling.rs-0641 */             profiler.alloc_string("IncrementalResultHashing");
/* FP:profiling.rs-0642 */         let query_blocked_event_kind = profiler.alloc_string("QueryBlocked");
/* FP:profiling.rs-0643 */         let query_cache_hit_event_kind = profiler.alloc_string("QueryCacheHit");
/* FP:profiling.rs-0644 */         let artifact_size_event_kind = profiler.alloc_string("ArtifactSize");
/* FP:profiling.rs-0645 */         let query_cache_hit_count_event_kind = profiler.alloc_string("QueryCacheHitCount");
/* FP:profiling.rs-0646 */ 
/* FP:profiling.rs-0647 */         let mut event_filter_mask = EventFilter::empty();
/* FP:profiling.rs-0648 */ 
/* FP:profiling.rs-0649 */         if let Some(event_filters) = event_filters {
/* FP:profiling.rs-0650 */             let mut unknown_events = vec![];
/* FP:profiling.rs-0651 */             for item in event_filters {
/* FP:profiling.rs-0652 */                 if let Some(&(_, mask)) =
/* FP:profiling.rs-0653 */                     EVENT_FILTERS_BY_NAME.iter().find(|&(name, _)| name == item)
/* FP:profiling.rs-0654 */                 {
/* FP:profiling.rs-0655 */                     event_filter_mask |= mask;
/* FP:profiling.rs-0656 */                 } else {
/* FP:profiling.rs-0657 */                     unknown_events.push(item.clone());
/* FP:profiling.rs-0658 */                 }
/* FP:profiling.rs-0659 */             }
/* FP:profiling.rs-0660 */ 
/* FP:profiling.rs-0661 */             // Warn about any unknown event names
/* FP:profiling.rs-0662 */             if !unknown_events.is_empty() {
/* FP:profiling.rs-0663 */                 unknown_events.sort();
/* FP:profiling.rs-0664 */                 unknown_events.dedup();
/* FP:profiling.rs-0665 */ 
/* FP:profiling.rs-0666 */                 warn!(
/* FP:profiling.rs-0667 */                     "Unknown self-profiler events specified: {}. Available options are: {}.",
/* FP:profiling.rs-0668 */                     unknown_events.join(", "),
/* FP:profiling.rs-0669 */                     EVENT_FILTERS_BY_NAME
/* FP:profiling.rs-0670 */                         .iter()
/* FP:profiling.rs-0671 */                         .map(|&(name, _)| name.to_string())
/* FP:profiling.rs-0672 */                         .collect::<Vec<_>>()
/* FP:profiling.rs-0673 */                         .join(", ")
/* FP:profiling.rs-0674 */                 );
/* FP:profiling.rs-0675 */             }
/* FP:profiling.rs-0676 */         } else {
/* FP:profiling.rs-0677 */             event_filter_mask = EventFilter::DEFAULT;
/* FP:profiling.rs-0678 */         }
/* FP:profiling.rs-0679 */ 
/* FP:profiling.rs-0680 */         Ok(SelfProfiler {
/* FP:profiling.rs-0681 */             profiler,
/* FP:profiling.rs-0682 */             event_filter_mask,
/* FP:profiling.rs-0683 */             string_cache: RwLock::new(FxHashMap::default()),
/* FP:profiling.rs-0684 */             query_event_kind,
/* FP:profiling.rs-0685 */             generic_activity_event_kind,
/* FP:profiling.rs-0686 */             incremental_load_result_event_kind,
/* FP:profiling.rs-0687 */             incremental_result_hashing_event_kind,
/* FP:profiling.rs-0688 */             query_blocked_event_kind,
/* FP:profiling.rs-0689 */             query_cache_hit_event_kind,
/* FP:profiling.rs-0690 */             artifact_size_event_kind,
/* FP:profiling.rs-0691 */             query_cache_hit_count_event_kind,
/* FP:profiling.rs-0692 */             query_hits: Default::default(),
/* FP:profiling.rs-0693 */         })
/* FP:profiling.rs-0694 */     }
/* FP:profiling.rs-0695 */ 
/* FP:profiling.rs-0696 */     /// Allocates a new string in the profiling data. Does not do any caching
/* FP:profiling.rs-0697 */     /// or deduplication.
/* FP:profiling.rs-0698 */     pub fn alloc_string<STR: SerializableString + ?Sized>(&self, s: &STR) -> StringId {
/* FP:profiling.rs-0699 */         self.profiler.alloc_string(s)
/* FP:profiling.rs-0700 */     }
/* FP:profiling.rs-0701 */ 
/* FP:profiling.rs-0702 */     /// Store a cache hit of a query invocation
/* FP:profiling.rs-0703 */     pub fn increment_query_cache_hit_counters(&self, id: QueryInvocationId) {
/* FP:profiling.rs-0704 */         // Fast path: assume that the query was already encountered before, and just record
/* FP:profiling.rs-0705 */         // a cache hit.
/* FP:profiling.rs-0706 */         let mut guard = self.query_hits.upgradable_read();
/* FP:profiling.rs-0707 */         let query_hits = &guard;
/* FP:profiling.rs-0708 */         let index = id.0 as usize;
/* FP:profiling.rs-0709 */         if index < query_hits.len() {
/* FP:profiling.rs-0710 */             // We only want to increment the count, no other synchronization is required
/* FP:profiling.rs-0711 */             query_hits[index].fetch_add(1, Ordering::Relaxed);
/* FP:profiling.rs-0712 */         } else {
/* FP:profiling.rs-0713 */             // If not, we need to extend the query hit map to the highest observed ID
/* FP:profiling.rs-0714 */             guard.with_upgraded(|vec| {
/* FP:profiling.rs-0715 */                 vec.resize_with(index + 1, || AtomicU64::new(0));
/* FP:profiling.rs-0716 */                 vec[index] = AtomicU64::from(1);
/* FP:profiling.rs-0717 */             });
/* FP:profiling.rs-0718 */         }
/* FP:profiling.rs-0719 */     }
/* FP:profiling.rs-0720 */ 
/* FP:profiling.rs-0721 */     /// Gets a `StringId` for the given string. This method makes sure that
/* FP:profiling.rs-0722 */     /// any strings going through it will only be allocated once in the
/* FP:profiling.rs-0723 */     /// profiling data.
/* FP:profiling.rs-0724 */     pub fn get_or_alloc_cached_string<A>(&self, s: A) -> StringId
/* FP:profiling.rs-0725 */     where
/* FP:profiling.rs-0726 */         A: Borrow<str> + Into<String>,
/* FP:profiling.rs-0727 */     {
/* FP:profiling.rs-0728 */         // Only acquire a read-lock first since we assume that the string is
/* FP:profiling.rs-0729 */         // already present in the common case.
/* FP:profiling.rs-0730 */         {
/* FP:profiling.rs-0731 */             let string_cache = self.string_cache.read();
/* FP:profiling.rs-0732 */ 
/* FP:profiling.rs-0733 */             if let Some(&id) = string_cache.get(s.borrow()) {
/* FP:profiling.rs-0734 */                 return id;
/* FP:profiling.rs-0735 */             }
/* FP:profiling.rs-0736 */         }
/* FP:profiling.rs-0737 */ 
/* FP:profiling.rs-0738 */         let mut string_cache = self.string_cache.write();
/* FP:profiling.rs-0739 */         // Check if the string has already been added in the small time window
/* FP:profiling.rs-0740 */         // between dropping the read lock and acquiring the write lock.
/* FP:profiling.rs-0741 */         match string_cache.entry(s.into()) {
/* FP:profiling.rs-0742 */             Entry::Occupied(e) => *e.get(),
/* FP:profiling.rs-0743 */             Entry::Vacant(e) => {
/* FP:profiling.rs-0744 */                 let string_id = self.profiler.alloc_string(&e.key()[..]);
/* FP:profiling.rs-0745 */                 *e.insert(string_id)
/* FP:profiling.rs-0746 */             }
/* FP:profiling.rs-0747 */         }
/* FP:profiling.rs-0748 */     }
/* FP:profiling.rs-0749 */ 
/* FP:profiling.rs-0750 */     pub fn map_query_invocation_id_to_string(&self, from: QueryInvocationId, to: StringId) {
/* FP:profiling.rs-0751 */         let from = StringId::new_virtual(from.0);
/* FP:profiling.rs-0752 */         self.profiler.map_virtual_to_concrete_string(from, to);
/* FP:profiling.rs-0753 */     }
/* FP:profiling.rs-0754 */ 
/* FP:profiling.rs-0755 */     pub fn bulk_map_query_invocation_id_to_single_string<I>(&self, from: I, to: StringId)
/* FP:profiling.rs-0756 */     where
/* FP:profiling.rs-0757 */         I: Iterator<Item = QueryInvocationId> + ExactSizeIterator,
/* FP:profiling.rs-0758 */     {
/* FP:profiling.rs-0759 */         let from = from.map(|qid| StringId::new_virtual(qid.0));
/* FP:profiling.rs-0760 */         self.profiler.bulk_map_virtual_to_single_concrete_string(from, to);
/* FP:profiling.rs-0761 */     }
/* FP:profiling.rs-0762 */ 
/* FP:profiling.rs-0763 */     pub fn query_key_recording_enabled(&self) -> bool {
/* FP:profiling.rs-0764 */         self.event_filter_mask.contains(EventFilter::QUERY_KEYS)
/* FP:profiling.rs-0765 */     }
/* FP:profiling.rs-0766 */ 
/* FP:profiling.rs-0767 */     pub fn event_id_builder(&self) -> EventIdBuilder<'_> {
/* FP:profiling.rs-0768 */         EventIdBuilder::new(&self.profiler)
/* FP:profiling.rs-0769 */     }
/* FP:profiling.rs-0770 */ }
/* FP:profiling.rs-0771 */ 
/* FP:profiling.rs-0772 */ #[must_use]
/* FP:profiling.rs-0773 */ pub struct TimingGuard<'a>(Option<measureme::TimingGuard<'a>>);
/* FP:profiling.rs-0774 */ 
/* FP:profiling.rs-0775 */ impl<'a> TimingGuard<'a> {
/* FP:profiling.rs-0776 */     #[inline]
/* FP:profiling.rs-0777 */     pub fn start(
/* FP:profiling.rs-0778 */         profiler: &'a SelfProfiler,
/* FP:profiling.rs-0779 */         event_kind: StringId,
/* FP:profiling.rs-0780 */         event_id: EventId,
/* FP:profiling.rs-0781 */     ) -> TimingGuard<'a> {
/* FP:profiling.rs-0782 */         let thread_id = get_thread_id();
/* FP:profiling.rs-0783 */         let raw_profiler = &profiler.profiler;
/* FP:profiling.rs-0784 */         let timing_guard =
/* FP:profiling.rs-0785 */             raw_profiler.start_recording_interval_event(event_kind, event_id, thread_id);
/* FP:profiling.rs-0786 */         TimingGuard(Some(timing_guard))
/* FP:profiling.rs-0787 */     }
/* FP:profiling.rs-0788 */ 
/* FP:profiling.rs-0789 */     #[inline]
/* FP:profiling.rs-0790 */     pub fn finish_with_query_invocation_id(self, query_invocation_id: QueryInvocationId) {
/* FP:profiling.rs-0791 */         if let Some(guard) = self.0 {
/* FP:profiling.rs-0792 */             outline(|| {
/* FP:profiling.rs-0793 */                 let event_id = StringId::new_virtual(query_invocation_id.0);
/* FP:profiling.rs-0794 */                 let event_id = EventId::from_virtual(event_id);
/* FP:profiling.rs-0795 */                 guard.finish_with_override_event_id(event_id);
/* FP:profiling.rs-0796 */             });
/* FP:profiling.rs-0797 */         }
/* FP:profiling.rs-0798 */     }
/* FP:profiling.rs-0799 */ 
/* FP:profiling.rs-0800 */     #[inline]
/* FP:profiling.rs-0801 */     pub fn none() -> TimingGuard<'a> {
/* FP:profiling.rs-0802 */         TimingGuard(None)
/* FP:profiling.rs-0803 */     }
/* FP:profiling.rs-0804 */ 
/* FP:profiling.rs-0805 */     #[inline(always)]
/* FP:profiling.rs-0806 */     pub fn run<R>(self, f: impl FnOnce() -> R) -> R {
/* FP:profiling.rs-0807 */         let _timer = self;
/* FP:profiling.rs-0808 */         f()
/* FP:profiling.rs-0809 */     }
/* FP:profiling.rs-0810 */ }
/* FP:profiling.rs-0811 */ 
/* FP:profiling.rs-0812 */ struct VerboseInfo {
/* FP:profiling.rs-0813 */     start_time: Instant,
/* FP:profiling.rs-0814 */     start_rss: Option<usize>,
/* FP:profiling.rs-0815 */     message: String,
/* FP:profiling.rs-0816 */     format: TimePassesFormat,
/* FP:profiling.rs-0817 */ }
/* FP:profiling.rs-0818 */ 
/* FP:profiling.rs-0819 */ #[must_use]
/* FP:profiling.rs-0820 */ pub struct VerboseTimingGuard<'a> {
/* FP:profiling.rs-0821 */     info: Option<VerboseInfo>,
/* FP:profiling.rs-0822 */     _guard: TimingGuard<'a>,
/* FP:profiling.rs-0823 */ }
/* FP:profiling.rs-0824 */ 
/* FP:profiling.rs-0825 */ impl<'a> VerboseTimingGuard<'a> {
/* FP:profiling.rs-0826 */     pub fn start(
/* FP:profiling.rs-0827 */         message_and_format: Option<(String, TimePassesFormat)>,
/* FP:profiling.rs-0828 */         _guard: TimingGuard<'a>,
/* FP:profiling.rs-0829 */     ) -> Self {
/* FP:profiling.rs-0830 */         VerboseTimingGuard {
/* FP:profiling.rs-0831 */             _guard,
/* FP:profiling.rs-0832 */             info: message_and_format.map(|(message, format)| VerboseInfo {
/* FP:profiling.rs-0833 */                 start_time: Instant::now(),
/* FP:profiling.rs-0834 */                 start_rss: get_resident_set_size(),
/* FP:profiling.rs-0835 */                 message,
/* FP:profiling.rs-0836 */                 format,
/* FP:profiling.rs-0837 */             }),
/* FP:profiling.rs-0838 */         }
/* FP:profiling.rs-0839 */     }
/* FP:profiling.rs-0840 */ 
/* FP:profiling.rs-0841 */     #[inline(always)]
/* FP:profiling.rs-0842 */     pub fn run<R>(self, f: impl FnOnce() -> R) -> R {
/* FP:profiling.rs-0843 */         let _timer = self;
/* FP:profiling.rs-0844 */         f()
/* FP:profiling.rs-0845 */     }
/* FP:profiling.rs-0846 */ }
/* FP:profiling.rs-0847 */ 
/* FP:profiling.rs-0848 */ impl Drop for VerboseTimingGuard<'_> {
/* FP:profiling.rs-0849 */     fn drop(&mut self) {
/* FP:profiling.rs-0850 */         if let Some(info) = &self.info {
/* FP:profiling.rs-0851 */             let end_rss = get_resident_set_size();
/* FP:profiling.rs-0852 */             let dur = info.start_time.elapsed();
/* FP:profiling.rs-0853 */             print_time_passes_entry(&info.message, dur, info.start_rss, end_rss, info.format);
/* FP:profiling.rs-0854 */         }
/* FP:profiling.rs-0855 */     }
/* FP:profiling.rs-0856 */ }
/* FP:profiling.rs-0857 */ 
/* FP:profiling.rs-0858 */ struct JsonTimePassesEntry<'a> {
/* FP:profiling.rs-0859 */     pass: &'a str,
/* FP:profiling.rs-0860 */     time: f64,
/* FP:profiling.rs-0861 */     start_rss: Option<usize>,
/* FP:profiling.rs-0862 */     end_rss: Option<usize>,
/* FP:profiling.rs-0863 */ }
/* FP:profiling.rs-0864 */ 
/* FP:profiling.rs-0865 */ impl Display for JsonTimePassesEntry<'_> {
/* FP:profiling.rs-0866 */     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:profiling.rs-0867 */         let Self { pass: what, time, start_rss, end_rss } = self;
/* FP:profiling.rs-0868 */         write!(f, r#"{{"pass":"{what}","time":{time},"rss_start":"#).unwrap();
/* FP:profiling.rs-0869 */         match start_rss {
/* FP:profiling.rs-0870 */             Some(rss) => write!(f, "{rss}")?,
/* FP:profiling.rs-0871 */             None => write!(f, "null")?,
/* FP:profiling.rs-0872 */         }
/* FP:profiling.rs-0873 */         write!(f, r#","rss_end":"#)?;
/* FP:profiling.rs-0874 */         match end_rss {
/* FP:profiling.rs-0875 */             Some(rss) => write!(f, "{rss}")?,
/* FP:profiling.rs-0876 */             None => write!(f, "null")?,
/* FP:profiling.rs-0877 */         }
/* FP:profiling.rs-0878 */         write!(f, "}}")?;
/* FP:profiling.rs-0879 */         Ok(())
/* FP:profiling.rs-0880 */     }
/* FP:profiling.rs-0881 */ }
/* FP:profiling.rs-0882 */ 
/* FP:profiling.rs-0883 */ pub fn print_time_passes_entry(
/* FP:profiling.rs-0884 */     what: &str,
/* FP:profiling.rs-0885 */     dur: Duration,
/* FP:profiling.rs-0886 */     start_rss: Option<usize>,
/* FP:profiling.rs-0887 */     end_rss: Option<usize>,
/* FP:profiling.rs-0888 */     format: TimePassesFormat,
/* FP:profiling.rs-0889 */ ) {
/* FP:profiling.rs-0890 */     match format {
/* FP:profiling.rs-0891 */         TimePassesFormat::Json => {
/* FP:profiling.rs-0892 */             let entry =
/* FP:profiling.rs-0893 */                 JsonTimePassesEntry { pass: what, time: dur.as_secs_f64(), start_rss, end_rss };
/* FP:profiling.rs-0894 */ 
/* FP:profiling.rs-0895 */             eprintln!(r#"time: {entry}"#);
/* FP:profiling.rs-0896 */             return;
/* FP:profiling.rs-0897 */         }
/* FP:profiling.rs-0898 */         TimePassesFormat::Text => (),
/* FP:profiling.rs-0899 */     }
/* FP:profiling.rs-0900 */ 
/* FP:profiling.rs-0901 */     // Print the pass if its duration is greater than 5 ms, or it changed the
/* FP:profiling.rs-0902 */     // measured RSS.
/* FP:profiling.rs-0903 */     let is_notable = || {
/* FP:profiling.rs-0904 */         if dur.as_millis() > 5 {
/* FP:profiling.rs-0905 */             return true;
/* FP:profiling.rs-0906 */         }
/* FP:profiling.rs-0907 */ 
/* FP:profiling.rs-0908 */         if let (Some(start_rss), Some(end_rss)) = (start_rss, end_rss) {
/* FP:profiling.rs-0909 */             let change_rss = end_rss.abs_diff(start_rss);
/* FP:profiling.rs-0910 */             if change_rss > 0 {
/* FP:profiling.rs-0911 */                 return true;
/* FP:profiling.rs-0912 */             }
/* FP:profiling.rs-0913 */         }
/* FP:profiling.rs-0914 */ 
/* FP:profiling.rs-0915 */         false
/* FP:profiling.rs-0916 */     };
/* FP:profiling.rs-0917 */     if !is_notable() {
/* FP:profiling.rs-0918 */         return;
/* FP:profiling.rs-0919 */     }
/* FP:profiling.rs-0920 */ 
/* FP:profiling.rs-0921 */     let rss_to_mb = |rss| (rss as f64 / 1_000_000.0).round() as usize;
/* FP:profiling.rs-0922 */     let rss_change_to_mb = |rss| (rss as f64 / 1_000_000.0).round() as i128;
/* FP:profiling.rs-0923 */ 
/* FP:profiling.rs-0924 */     let mem_string = match (start_rss, end_rss) {
/* FP:profiling.rs-0925 */         (Some(start_rss), Some(end_rss)) => {
/* FP:profiling.rs-0926 */             let change_rss = end_rss as i128 - start_rss as i128;
/* FP:profiling.rs-0927 */ 
/* FP:profiling.rs-0928 */             format!(
/* FP:profiling.rs-0929 */                 "; rss: {:>4}MB -> {:>4}MB ({:>+5}MB)",
/* FP:profiling.rs-0930 */                 rss_to_mb(start_rss),
/* FP:profiling.rs-0931 */                 rss_to_mb(end_rss),
/* FP:profiling.rs-0932 */                 rss_change_to_mb(change_rss),
/* FP:profiling.rs-0933 */             )
/* FP:profiling.rs-0934 */         }
/* FP:profiling.rs-0935 */         (Some(start_rss), None) => format!("; rss start: {:>4}MB", rss_to_mb(start_rss)),
/* FP:profiling.rs-0936 */         (None, Some(end_rss)) => format!("; rss end: {:>4}MB", rss_to_mb(end_rss)),
/* FP:profiling.rs-0937 */         (None, None) => String::new(),
/* FP:profiling.rs-0938 */     };
/* FP:profiling.rs-0939 */ 
/* FP:profiling.rs-0940 */     eprintln!("time: {:>7}{}\t{}", duration_to_secs_str(dur), mem_string, what);
/* FP:profiling.rs-0941 */ }
/* FP:profiling.rs-0942 */ 
/* FP:profiling.rs-0943 */ // Hack up our own formatting for the duration to make it easier for scripts
/* FP:profiling.rs-0944 */ // to parse (always use the same number of decimal places and the same unit).
/* FP:profiling.rs-0945 */ pub fn duration_to_secs_str(dur: std::time::Duration) -> String {
/* FP:profiling.rs-0946 */     format!("{:.3}", dur.as_secs_f64())
/* FP:profiling.rs-0947 */ }
/* FP:profiling.rs-0948 */ 
/* FP:profiling.rs-0949 */ fn get_thread_id() -> u32 {
/* FP:profiling.rs-0950 */     std::thread::current().id().as_u64().get() as u32
/* FP:profiling.rs-0951 */ }
/* FP:profiling.rs-0952 */ 
/* FP:profiling.rs-0953 */ // Memory reporting
/* FP:profiling.rs-0954 */ cfg_select! {
/* FP:profiling.rs-0955 */     windows => {
/* FP:profiling.rs-0956 */         pub fn get_resident_set_size() -> Option<usize> {
/* FP:profiling.rs-0957 */             use windows::{
/* FP:profiling.rs-0958 */                 Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
/* FP:profiling.rs-0959 */                 Win32::System::Threading::GetCurrentProcess,
/* FP:profiling.rs-0960 */             };
/* FP:profiling.rs-0961 */ 
/* FP:profiling.rs-0962 */             let mut pmc = PROCESS_MEMORY_COUNTERS::default();
/* FP:profiling.rs-0963 */             let pmc_size = size_of_val(&pmc);
/* FP:profiling.rs-0964 */             unsafe {
/* FP:profiling.rs-0965 */                 K32GetProcessMemoryInfo(
/* FP:profiling.rs-0966 */                     GetCurrentProcess(),
/* FP:profiling.rs-0967 */                     &mut pmc,
/* FP:profiling.rs-0968 */                     pmc_size as u32,
/* FP:profiling.rs-0969 */                 )
/* FP:profiling.rs-0970 */             }
/* FP:profiling.rs-0971 */             .ok()
/* FP:profiling.rs-0972 */             .ok()?;
/* FP:profiling.rs-0973 */ 
/* FP:profiling.rs-0974 */             Some(pmc.WorkingSetSize)
/* FP:profiling.rs-0975 */         }
/* FP:profiling.rs-0976 */     }
/* FP:profiling.rs-0977 */     target_os = "macos" => {
/* FP:profiling.rs-0978 */         pub fn get_resident_set_size() -> Option<usize> {
/* FP:profiling.rs-0979 */             use libc::{c_int, c_void, getpid, proc_pidinfo, proc_taskinfo, PROC_PIDTASKINFO};
/* FP:profiling.rs-0980 */             use std::mem;
/* FP:profiling.rs-0981 */             const PROC_TASKINFO_SIZE: c_int = size_of::<proc_taskinfo>() as c_int;
/* FP:profiling.rs-0982 */ 
/* FP:profiling.rs-0983 */             unsafe {
/* FP:profiling.rs-0984 */                 let mut info: proc_taskinfo = mem::zeroed();
/* FP:profiling.rs-0985 */                 let info_ptr = &mut info as *mut proc_taskinfo as *mut c_void;
/* FP:profiling.rs-0986 */                 let pid = getpid() as c_int;
/* FP:profiling.rs-0987 */                 let ret = proc_pidinfo(pid, PROC_PIDTASKINFO, 0, info_ptr, PROC_TASKINFO_SIZE);
/* FP:profiling.rs-0988 */                 if ret == PROC_TASKINFO_SIZE {
/* FP:profiling.rs-0989 */                     Some(info.pti_resident_size as usize)
/* FP:profiling.rs-0990 */                 } else {
/* FP:profiling.rs-0991 */                     None
/* FP:profiling.rs-0992 */                 }
/* FP:profiling.rs-0993 */             }
/* FP:profiling.rs-0994 */         }
/* FP:profiling.rs-0995 */     }
/* FP:profiling.rs-0996 */     unix => {
/* FP:profiling.rs-0997 */         pub fn get_resident_set_size() -> Option<usize> {
/* FP:profiling.rs-0998 */             let field = 1;
/* FP:profiling.rs-0999 */             let contents = fs::read("/proc/self/statm").ok()?;
/* FP:profiling.rs-1000 */             let contents = String::from_utf8(contents).ok()?;
/* FP:profiling.rs-1001 */             let s = contents.split_whitespace().nth(field)?;
/* FP:profiling.rs-1002 */             let npages = s.parse::<usize>().ok()?;
/* FP:profiling.rs-1003 */             Some(npages * 4096)
/* FP:profiling.rs-1004 */         }
/* FP:profiling.rs-1005 */     }
/* FP:profiling.rs-1006 */     _ => {
/* FP:profiling.rs-1007 */         pub fn get_resident_set_size() -> Option<usize> {
/* FP:profiling.rs-1008 */             None
/* FP:profiling.rs-1009 */         }
/* FP:profiling.rs-1010 */     }
/* FP:profiling.rs-1011 */ }
/* FP:profiling.rs-1012 */ 
/* FP:profiling.rs-1013 */ #[cfg(test)]