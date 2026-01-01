/* FP:symbol.rs-0001 */ // An "interner" is a data structure that associates values with usize tags and
/* FP:symbol.rs-0002 */ // allows bidirectional lookup; i.e., given a value, one can easily find the
/* FP:symbol.rs-0003 */ // type, and vice versa.
/* FP:symbol.rs-0004 */ 
/* FP:symbol.rs-0005 */ use std::hash::{Hash, Hasher};
/* FP:symbol.rs-0006 */ use std::ops::Deref;
/* FP:symbol.rs-0007 */ use std::{fmt, str};
/* FP:symbol.rs-0008 */ 
/* FP:symbol.rs-0009 */ use rustc_arena::DroplessArena;
/* FP:symbol.rs-0010 */ use crate::rustc_data_structures::fx::{FxHashSet, FxIndexSet};
/* FP:symbol.rs-0011 */ use crate::rustc_data_structures::stable_hasher::{
/* FP:symbol.rs-0012 */     HashStable, StableCompare, StableHasher, ToStableHashKey,
/* FP:symbol.rs-0013 */ };
/* FP:symbol.rs-0014 */ use crate::rustc_data_structures::sync::Lock;
/* FP:symbol.rs-0015 */ use rustc_macros::{Decodable, Encodable, HashStable_Generic, symbols};
/* FP:symbol.rs-0016 */ 
/* FP:symbol.rs-0017 */ use crate::{DUMMY_SP, Edition, Span, with_session_globals};
/* FP:symbol.rs-0018 */ 
/* FP:symbol.rs-0019 */ #[cfg(test)]
/* FP:symbol.rs-0021 */ 
/* FP:symbol.rs-0022 */ // The proc macro code for this is in `compiler/rustc_macros/src/symbols.rs`.
/* FP:symbol.rs-0023 */ symbols! {
/* FP:symbol.rs-0024 */     // This list includes things that are definitely keywords (e.g. `if`), a
/* FP:symbol.rs-0025 */     // few things that are definitely not keywords (e.g. `{{root}}`) and things
/* FP:symbol.rs-0026 */     // where there is disagreement between people and/or documents (such as the
/* FP:symbol.rs-0027 */     // Rust Reference) about whether it is a keyword (e.g. `_`).
/* FP:symbol.rs-0028 */     //
/* FP:symbol.rs-0029 */     // If you modify this list, adjust any relevant `Symbol::{is,can_be}_*`
/* FP:symbol.rs-0030 */     // predicates and `used_keywords`. Also consider adding new keywords to the
/* FP:symbol.rs-0031 */     // `ui/parser/raw/raw-idents.rs` test.
/* FP:symbol.rs-0032 */     Keywords {
/* FP:symbol.rs-0033 */         // Special reserved identifiers used internally for unnamed method
/* FP:symbol.rs-0034 */         // parameters, crate root module, etc.
/* FP:symbol.rs-0035 */         // Matching predicates: `is_special`/`is_reserved`
/* FP:symbol.rs-0036 */         //
/* FP:symbol.rs-0037 */         // tidy-alphabetical-start
/* FP:symbol.rs-0038 */         DollarCrate:        "$crate",
/* FP:symbol.rs-0039 */         PathRoot:           "{{root}}",
/* FP:symbol.rs-0040 */         Underscore:         "_",
/* FP:symbol.rs-0041 */         // tidy-alphabetical-end
/* FP:symbol.rs-0042 */ 
/* FP:symbol.rs-0043 */         // Keywords that are used in stable Rust.
/* FP:symbol.rs-0044 */         // Matching predicates: `is_used_keyword_always`/`is_reserved`
/* FP:symbol.rs-0045 */         // tidy-alphabetical-start
/* FP:symbol.rs-0046 */         As:                 "as",
/* FP:symbol.rs-0047 */         Break:              "break",
/* FP:symbol.rs-0048 */         Const:              "const",
/* FP:symbol.rs-0049 */         Continue:           "continue",
/* FP:symbol.rs-0050 */         Crate:              "crate",
/* FP:symbol.rs-0051 */         Else:               "else",
/* FP:symbol.rs-0052 */         Enum:               "enum",
/* FP:symbol.rs-0053 */         Extern:             "extern",
/* FP:symbol.rs-0054 */         False:              "false",
/* FP:symbol.rs-0055 */         Fn:                 "fn",
/* FP:symbol.rs-0056 */         For:                "for",
/* FP:symbol.rs-0057 */         If:                 "if",
/* FP:symbol.rs-0058 */         Impl:               "impl",
/* FP:symbol.rs-0059 */         In:                 "in",
/* FP:symbol.rs-0060 */         Let:                "let",
/* FP:symbol.rs-0061 */         Loop:               "loop",
/* FP:symbol.rs-0062 */         Match:              "match",
/* FP:symbol.rs-0063 */         Mod:                "mod",
/* FP:symbol.rs-0064 */         Move:               "move",
/* FP:symbol.rs-0065 */         Mut:                "mut",
/* FP:symbol.rs-0066 */         Pub:                "pub",
/* FP:symbol.rs-0067 */         Ref:                "ref",
/* FP:symbol.rs-0068 */         Return:             "return",
/* FP:symbol.rs-0069 */         SelfLower:          "self",
/* FP:symbol.rs-0070 */         SelfUpper:          "Self",
/* FP:symbol.rs-0071 */         Static:             "static",
/* FP:symbol.rs-0072 */         Struct:             "struct",
/* FP:symbol.rs-0073 */         Super:              "super",
/* FP:symbol.rs-0074 */         Trait:              "trait",
/* FP:symbol.rs-0075 */         True:               "true",
/* FP:symbol.rs-0076 */         Type:               "type",
/* FP:symbol.rs-0077 */         Unsafe:             "unsafe",
/* FP:symbol.rs-0078 */         Use:                "use",
/* FP:symbol.rs-0079 */         Where:              "where",
/* FP:symbol.rs-0080 */         While:              "while",
/* FP:symbol.rs-0081 */         // tidy-alphabetical-end
/* FP:symbol.rs-0082 */ 
/* FP:symbol.rs-0083 */         // Keywords that are used in unstable Rust or reserved for future use.
/* FP:symbol.rs-0084 */         // Matching predicates: `is_unused_keyword_always`/`is_reserved`
/* FP:symbol.rs-0085 */         // tidy-alphabetical-start
/* FP:symbol.rs-0086 */         Abstract:           "abstract",
/* FP:symbol.rs-0087 */         Become:             "become",
/* FP:symbol.rs-0088 */         Box:                "box",
/* FP:symbol.rs-0089 */         Do:                 "do",
/* FP:symbol.rs-0090 */         Final:              "final",
/* FP:symbol.rs-0091 */         Macro:              "macro",
/* FP:symbol.rs-0092 */         Override:           "override",
/* FP:symbol.rs-0093 */         Priv:               "priv",
/* FP:symbol.rs-0094 */         Typeof:             "typeof",
/* FP:symbol.rs-0095 */         Unsized:            "unsized",
/* FP:symbol.rs-0096 */         Virtual:            "virtual",
/* FP:symbol.rs-0097 */         Yield:              "yield",
/* FP:symbol.rs-0098 */         // tidy-alphabetical-end
/* FP:symbol.rs-0099 */ 
/* FP:symbol.rs-0100 */         // Edition-specific keywords that are used in stable Rust.
/* FP:symbol.rs-0101 */         // Matching predicates: `is_used_keyword_conditional`/`is_reserved` (if
/* FP:symbol.rs-0102 */         // the edition suffices)
/* FP:symbol.rs-0103 */         // tidy-alphabetical-start
/* FP:symbol.rs-0104 */         Async:              "async", // >= 2018 Edition only
/* FP:symbol.rs-0105 */         Await:              "await", // >= 2018 Edition only
/* FP:symbol.rs-0106 */         Dyn:                "dyn", // >= 2018 Edition only
/* FP:symbol.rs-0107 */         // tidy-alphabetical-end
/* FP:symbol.rs-0108 */ 
/* FP:symbol.rs-0109 */         // Edition-specific keywords that are used in unstable Rust or reserved for future use.
/* FP:symbol.rs-0110 */         // Matching predicates: `is_unused_keyword_conditional`/`is_reserved` (if
/* FP:symbol.rs-0111 */         // the edition suffices)
/* FP:symbol.rs-0112 */         // tidy-alphabetical-start
/* FP:symbol.rs-0113 */         Gen:                "gen", // >= 2024 Edition only
/* FP:symbol.rs-0114 */         Try:                "try", // >= 2018 Edition only
/* FP:symbol.rs-0115 */         // tidy-alphabetical-end
/* FP:symbol.rs-0116 */ 
/* FP:symbol.rs-0117 */         // "Lifetime keywords": regular keywords with a leading `'`.
/* FP:symbol.rs-0118 */         // Matching predicates: none
/* FP:symbol.rs-0119 */         // tidy-alphabetical-start
/* FP:symbol.rs-0120 */         StaticLifetime:     "'static",
/* FP:symbol.rs-0121 */         UnderscoreLifetime: "'_",
/* FP:symbol.rs-0122 */         // tidy-alphabetical-end
/* FP:symbol.rs-0123 */ 
/* FP:symbol.rs-0124 */         // Weak keywords, have special meaning only in specific contexts.
/* FP:symbol.rs-0125 */         // Matching predicates: `is_weak`
/* FP:symbol.rs-0126 */         // tidy-alphabetical-start
/* FP:symbol.rs-0127 */         Auto:               "auto",
/* FP:symbol.rs-0128 */         Builtin:            "builtin",
/* FP:symbol.rs-0129 */         Catch:              "catch",
/* FP:symbol.rs-0130 */         ContractEnsures:    "contract_ensures",
/* FP:symbol.rs-0131 */         ContractRequires:   "contract_requires",
/* FP:symbol.rs-0132 */         Default:            "default",
/* FP:symbol.rs-0133 */         MacroRules:         "macro_rules",
/* FP:symbol.rs-0134 */         Raw:                "raw",
/* FP:symbol.rs-0135 */         Reuse:              "reuse",
/* FP:symbol.rs-0136 */         Safe:               "safe",
/* FP:symbol.rs-0137 */         Union:              "union",
/* FP:symbol.rs-0138 */         Yeet:               "yeet",
/* FP:symbol.rs-0139 */         // tidy-alphabetical-end
/* FP:symbol.rs-0140 */     }
/* FP:symbol.rs-0141 */ 
/* FP:symbol.rs-0142 */     // Pre-interned symbols that can be referred to with `crate::rustc_span::sym::*`.
/* FP:symbol.rs-0143 */     //
/* FP:symbol.rs-0144 */     // The symbol is the stringified identifier unless otherwise specified, in
/* FP:symbol.rs-0145 */     // which case the name should mention the non-identifier punctuation.
/* FP:symbol.rs-0146 */     // E.g. `sym::proc_dash_macro` represents "proc-macro", and it shouldn't be
/* FP:symbol.rs-0147 */     // called `sym::proc_macro` because then it's easy to mistakenly think it
/* FP:symbol.rs-0148 */     // represents "proc_macro".
/* FP:symbol.rs-0149 */     //
/* FP:symbol.rs-0150 */     // As well as the symbols listed, there are symbols for the strings
/* FP:symbol.rs-0151 */     // "0", "1", ..., "9", which are accessible via `sym::integer`.
/* FP:symbol.rs-0152 */     //
/* FP:symbol.rs-0153 */     // There is currently no checking that all symbols are used; that would be
/* FP:symbol.rs-0154 */     // nice to have.
/* FP:symbol.rs-0155 */     Symbols {
/* FP:symbol.rs-0156 */         // tidy-alphabetical-start
/* FP:symbol.rs-0157 */         Abi,
/* FP:symbol.rs-0158 */         AcqRel,
/* FP:symbol.rs-0159 */         Acquire,
/* FP:symbol.rs-0160 */         Any,
/* FP:symbol.rs-0161 */         Arc,
/* FP:symbol.rs-0162 */         ArcWeak,
/* FP:symbol.rs-0163 */         Argument,
/* FP:symbol.rs-0164 */         ArrayIntoIter,
/* FP:symbol.rs-0165 */         AsMut,
/* FP:symbol.rs-0166 */         AsRef,
/* FP:symbol.rs-0167 */         AssertParamIsClone,
/* FP:symbol.rs-0168 */         AssertParamIsCopy,
/* FP:symbol.rs-0169 */         AssertParamIsEq,
/* FP:symbol.rs-0170 */         AsyncGenFinished,
/* FP:symbol.rs-0171 */         AsyncGenPending,
/* FP:symbol.rs-0172 */         AsyncGenReady,
/* FP:symbol.rs-0173 */         AtomicBool,
/* FP:symbol.rs-0174 */         AtomicI8,
/* FP:symbol.rs-0175 */         AtomicI16,
/* FP:symbol.rs-0176 */         AtomicI32,
/* FP:symbol.rs-0177 */         AtomicI64,
/* FP:symbol.rs-0178 */         AtomicI128,
/* FP:symbol.rs-0179 */         AtomicIsize,
/* FP:symbol.rs-0180 */         AtomicPtr,
/* FP:symbol.rs-0181 */         AtomicU8,
/* FP:symbol.rs-0182 */         AtomicU16,
/* FP:symbol.rs-0183 */         AtomicU32,
/* FP:symbol.rs-0184 */         AtomicU64,
/* FP:symbol.rs-0185 */         AtomicU128,
/* FP:symbol.rs-0186 */         AtomicUsize,
/* FP:symbol.rs-0187 */         BTreeEntry,
/* FP:symbol.rs-0188 */         BTreeMap,
/* FP:symbol.rs-0189 */         BTreeSet,
/* FP:symbol.rs-0190 */         BinaryHeap,
/* FP:symbol.rs-0191 */         Borrow,
/* FP:symbol.rs-0192 */         BorrowMut,
/* FP:symbol.rs-0193 */         Break,
/* FP:symbol.rs-0194 */         C,
/* FP:symbol.rs-0195 */         CStr,
/* FP:symbol.rs-0196 */         C_dash_unwind: "C-unwind",
/* FP:symbol.rs-0197 */         CallOnceFuture,
/* FP:symbol.rs-0198 */         CallRefFuture,
/* FP:symbol.rs-0199 */         Capture,
/* FP:symbol.rs-0200 */         Cell,
/* FP:symbol.rs-0201 */         Center,
/* FP:symbol.rs-0202 */         Child,
/* FP:symbol.rs-0203 */         Cleanup,
/* FP:symbol.rs-0204 */         Clone,
/* FP:symbol.rs-0205 */         CoercePointee,
/* FP:symbol.rs-0206 */         CoercePointeeValidated,
/* FP:symbol.rs-0207 */         CoerceUnsized,
/* FP:symbol.rs-0208 */         Command,
/* FP:symbol.rs-0209 */         ConstParamTy,
/* FP:symbol.rs-0210 */         ConstParamTy_,
/* FP:symbol.rs-0211 */         Context,
/* FP:symbol.rs-0212 */         Continue,
/* FP:symbol.rs-0213 */         ControlFlow,
/* FP:symbol.rs-0214 */         Copy,
/* FP:symbol.rs-0215 */         Cow,
/* FP:symbol.rs-0216 */         Debug,
/* FP:symbol.rs-0217 */         DebugStruct,
/* FP:symbol.rs-0218 */         Decodable,
/* FP:symbol.rs-0219 */         Decoder,
/* FP:symbol.rs-0220 */         Default,
/* FP:symbol.rs-0221 */         Deref,
/* FP:symbol.rs-0222 */         DiagMessage,
/* FP:symbol.rs-0223 */         Diagnostic,
/* FP:symbol.rs-0224 */         DirBuilder,
/* FP:symbol.rs-0225 */         DispatchFromDyn,
/* FP:symbol.rs-0226 */         Display,
/* FP:symbol.rs-0227 */         DoubleEndedIterator,
/* FP:symbol.rs-0228 */         Duration,
/* FP:symbol.rs-0229 */         Encodable,
/* FP:symbol.rs-0230 */         Encoder,
/* FP:symbol.rs-0231 */         Enumerate,
/* FP:symbol.rs-0232 */         Eq,
/* FP:symbol.rs-0233 */         Equal,
/* FP:symbol.rs-0234 */         Err,
/* FP:symbol.rs-0235 */         Error,
/* FP:symbol.rs-0236 */         File,
/* FP:symbol.rs-0237 */         FileType,
/* FP:symbol.rs-0238 */         FmtArgumentsNew,
/* FP:symbol.rs-0239 */         Fn,
/* FP:symbol.rs-0240 */         FnMut,
/* FP:symbol.rs-0241 */         FnOnce,
/* FP:symbol.rs-0242 */         Formatter,
/* FP:symbol.rs-0243 */         Forward,
/* FP:symbol.rs-0244 */         From,
/* FP:symbol.rs-0245 */         FromIterator,
/* FP:symbol.rs-0246 */         FromResidual,
/* FP:symbol.rs-0247 */         FsOpenOptions,
/* FP:symbol.rs-0248 */         FsPermissions,
/* FP:symbol.rs-0249 */         FusedIterator,
/* FP:symbol.rs-0250 */         Future,
/* FP:symbol.rs-0251 */         GlobalAlloc,
/* FP:symbol.rs-0252 */         Hash,
/* FP:symbol.rs-0253 */         HashMap,
/* FP:symbol.rs-0254 */         HashMapEntry,
/* FP:symbol.rs-0255 */         HashSet,
/* FP:symbol.rs-0256 */         Hasher,
/* FP:symbol.rs-0257 */         Implied,
/* FP:symbol.rs-0258 */         InCleanup,
/* FP:symbol.rs-0259 */         IndexOutput,
/* FP:symbol.rs-0260 */         Input,
/* FP:symbol.rs-0261 */         Instant,
/* FP:symbol.rs-0262 */         Into,
/* FP:symbol.rs-0263 */         IntoFuture,
/* FP:symbol.rs-0264 */         IntoIterator,
/* FP:symbol.rs-0265 */         IoBufRead,
/* FP:symbol.rs-0266 */         IoLines,
/* FP:symbol.rs-0267 */         IoRead,
/* FP:symbol.rs-0268 */         IoSeek,
/* FP:symbol.rs-0269 */         IoWrite,
/* FP:symbol.rs-0270 */         IpAddr,
/* FP:symbol.rs-0271 */         Ipv4Addr,
/* FP:symbol.rs-0272 */         Ipv6Addr,
/* FP:symbol.rs-0273 */         IrTyKind,
/* FP:symbol.rs-0274 */         Is,
/* FP:symbol.rs-0275 */         Item,
/* FP:symbol.rs-0276 */         ItemContext,
/* FP:symbol.rs-0277 */         IterEmpty,
/* FP:symbol.rs-0278 */         IterOnce,
/* FP:symbol.rs-0279 */         IterPeekable,
/* FP:symbol.rs-0280 */         Iterator,
/* FP:symbol.rs-0281 */         IteratorItem,
/* FP:symbol.rs-0282 */         IteratorMap,
/* FP:symbol.rs-0283 */         Layout,
/* FP:symbol.rs-0284 */         Left,
/* FP:symbol.rs-0285 */         LinkedList,
/* FP:symbol.rs-0286 */         LintDiagnostic,
/* FP:symbol.rs-0287 */         LintPass,
/* FP:symbol.rs-0288 */         LocalKey,
/* FP:symbol.rs-0289 */         Mutex,
/* FP:symbol.rs-0290 */         MutexGuard,
/* FP:symbol.rs-0291 */         N,
/* FP:symbol.rs-0292 */         NonNull,
/* FP:symbol.rs-0293 */         NonZero,
/* FP:symbol.rs-0294 */         None,
/* FP:symbol.rs-0295 */         Normal,
/* FP:symbol.rs-0296 */         Ok,
/* FP:symbol.rs-0297 */         Option,
/* FP:symbol.rs-0298 */         Ord,
/* FP:symbol.rs-0299 */         Ordering,
/* FP:symbol.rs-0300 */         OsStr,
/* FP:symbol.rs-0301 */         OsString,
/* FP:symbol.rs-0302 */         Output,
/* FP:symbol.rs-0303 */         Param,
/* FP:symbol.rs-0304 */         ParamSet,
/* FP:symbol.rs-0305 */         PartialEq,
/* FP:symbol.rs-0306 */         PartialOrd,
/* FP:symbol.rs-0307 */         Path,
/* FP:symbol.rs-0308 */         PathBuf,
/* FP:symbol.rs-0309 */         Pending,
/* FP:symbol.rs-0310 */         PinCoerceUnsized,
/* FP:symbol.rs-0311 */         Pointer,
/* FP:symbol.rs-0312 */         Poll,
/* FP:symbol.rs-0313 */         ProcMacro,
/* FP:symbol.rs-0314 */         ProceduralMasqueradeDummyType,
/* FP:symbol.rs-0315 */         Range,
/* FP:symbol.rs-0316 */         RangeBounds,
/* FP:symbol.rs-0317 */         RangeCopy,
/* FP:symbol.rs-0318 */         RangeFrom,
/* FP:symbol.rs-0319 */         RangeFromCopy,
/* FP:symbol.rs-0320 */         RangeFull,
/* FP:symbol.rs-0321 */         RangeInclusive,
/* FP:symbol.rs-0322 */         RangeInclusiveCopy,
/* FP:symbol.rs-0323 */         RangeMax,
/* FP:symbol.rs-0324 */         RangeMin,
/* FP:symbol.rs-0325 */         RangeSub,
/* FP:symbol.rs-0326 */         RangeTo,
/* FP:symbol.rs-0327 */         RangeToInclusive,
/* FP:symbol.rs-0328 */         RangeToInclusiveCopy,
/* FP:symbol.rs-0329 */         Rc,
/* FP:symbol.rs-0330 */         RcWeak,
/* FP:symbol.rs-0331 */         Ready,
/* FP:symbol.rs-0332 */         Receiver,
/* FP:symbol.rs-0333 */         RefCell,
/* FP:symbol.rs-0334 */         RefCellRef,
/* FP:symbol.rs-0335 */         RefCellRefMut,
/* FP:symbol.rs-0336 */         Relaxed,
/* FP:symbol.rs-0337 */         Release,
/* FP:symbol.rs-0338 */         Result,
/* FP:symbol.rs-0339 */         ResumeTy,
/* FP:symbol.rs-0340 */         Return,
/* FP:symbol.rs-0341 */         Reverse,
/* FP:symbol.rs-0342 */         Right,
/* FP:symbol.rs-0343 */         Rust,
/* FP:symbol.rs-0344 */         RustaceansAreAwesome,
/* FP:symbol.rs-0345 */         RwLock,
/* FP:symbol.rs-0346 */         RwLockReadGuard,
/* FP:symbol.rs-0347 */         RwLockWriteGuard,
/* FP:symbol.rs-0348 */         Saturating,
/* FP:symbol.rs-0349 */         SeekFrom,
/* FP:symbol.rs-0350 */         SelfTy,
/* FP:symbol.rs-0351 */         Send,
/* FP:symbol.rs-0352 */         SeqCst,
/* FP:symbol.rs-0353 */         Sized,
/* FP:symbol.rs-0354 */         SliceIndex,
/* FP:symbol.rs-0355 */         SliceIter,
/* FP:symbol.rs-0356 */         Some,
/* FP:symbol.rs-0357 */         SpanCtxt,
/* FP:symbol.rs-0358 */         Stdin,
/* FP:symbol.rs-0359 */         String,
/* FP:symbol.rs-0360 */         StructuralPartialEq,
/* FP:symbol.rs-0361 */         SubdiagMessage,
/* FP:symbol.rs-0362 */         Subdiagnostic,
/* FP:symbol.rs-0363 */         SymbolIntern,
/* FP:symbol.rs-0364 */         Sync,
/* FP:symbol.rs-0365 */         SyncUnsafeCell,
/* FP:symbol.rs-0366 */         T,
/* FP:symbol.rs-0367 */         Target,
/* FP:symbol.rs-0368 */         This,
/* FP:symbol.rs-0369 */         ToOwned,
/* FP:symbol.rs-0370 */         ToString,
/* FP:symbol.rs-0371 */         TokenStream,
/* FP:symbol.rs-0372 */         Trait,
/* FP:symbol.rs-0373 */         Try,
/* FP:symbol.rs-0374 */         TryCaptureGeneric,
/* FP:symbol.rs-0375 */         TryCapturePrintable,
/* FP:symbol.rs-0376 */         TryFrom,
/* FP:symbol.rs-0377 */         TryInto,
/* FP:symbol.rs-0378 */         Ty,
/* FP:symbol.rs-0379 */         TyCtxt,
/* FP:symbol.rs-0380 */         TyKind,
/* FP:symbol.rs-0381 */         Unknown,
/* FP:symbol.rs-0382 */         Unsize,
/* FP:symbol.rs-0383 */         UnsizedConstParamTy,
/* FP:symbol.rs-0384 */         Upvars,
/* FP:symbol.rs-0385 */         Vec,
/* FP:symbol.rs-0386 */         VecDeque,
/* FP:symbol.rs-0387 */         Waker,
/* FP:symbol.rs-0388 */         Wrapper,
/* FP:symbol.rs-0389 */         Wrapping,
/* FP:symbol.rs-0390 */         Yield,
/* FP:symbol.rs-0391 */         _DECLS,
/* FP:symbol.rs-0392 */         __D,
/* FP:symbol.rs-0393 */         __H,
/* FP:symbol.rs-0394 */         __S,
/* FP:symbol.rs-0395 */         __T,
/* FP:symbol.rs-0396 */         __awaitee,
/* FP:symbol.rs-0397 */         __try_var,
/* FP:symbol.rs-0398 */         _t,
/* FP:symbol.rs-0399 */         _task_context,
/* FP:symbol.rs-0400 */         a32,
/* FP:symbol.rs-0401 */         aarch64_target_feature,
/* FP:symbol.rs-0402 */         aarch64_unstable_target_feature,
/* FP:symbol.rs-0403 */         aarch64_ver_target_feature,
/* FP:symbol.rs-0404 */         abi,
/* FP:symbol.rs-0405 */         abi_amdgpu_kernel,
/* FP:symbol.rs-0406 */         abi_avr_interrupt,
/* FP:symbol.rs-0407 */         abi_c_cmse_nonsecure_call,
/* FP:symbol.rs-0408 */         abi_cmse_nonsecure_call,
/* FP:symbol.rs-0409 */         abi_custom,
/* FP:symbol.rs-0410 */         abi_efiapi,
/* FP:symbol.rs-0411 */         abi_gpu_kernel,
/* FP:symbol.rs-0412 */         abi_msp430_interrupt,
/* FP:symbol.rs-0413 */         abi_ptx,
/* FP:symbol.rs-0414 */         abi_riscv_interrupt,
/* FP:symbol.rs-0415 */         abi_sysv64,
/* FP:symbol.rs-0416 */         abi_thiscall,
/* FP:symbol.rs-0417 */         abi_unadjusted,
/* FP:symbol.rs-0418 */         abi_vectorcall,
/* FP:symbol.rs-0419 */         abi_x86_interrupt,
/* FP:symbol.rs-0420 */         abort,
/* FP:symbol.rs-0421 */         add,
/* FP:symbol.rs-0422 */         add_assign,
/* FP:symbol.rs-0423 */         add_with_overflow,
/* FP:symbol.rs-0424 */         address,
/* FP:symbol.rs-0425 */         adt_const_params,
/* FP:symbol.rs-0426 */         advanced_slice_patterns,
/* FP:symbol.rs-0427 */         adx_target_feature,
/* FP:symbol.rs-0428 */         aes,
/* FP:symbol.rs-0429 */         aggregate_raw_ptr,
/* FP:symbol.rs-0430 */         alias,
/* FP:symbol.rs-0431 */         align,
/* FP:symbol.rs-0432 */         align_of,
/* FP:symbol.rs-0433 */         align_of_val,
/* FP:symbol.rs-0434 */         alignment,
/* FP:symbol.rs-0435 */         all,
/* FP:symbol.rs-0436 */         alloc,
/* FP:symbol.rs-0437 */         alloc_error_handler,
/* FP:symbol.rs-0438 */         alloc_layout,
/* FP:symbol.rs-0439 */         alloc_zeroed,
/* FP:symbol.rs-0440 */         allocator,
/* FP:symbol.rs-0441 */         allocator_api,
/* FP:symbol.rs-0442 */         allocator_internals,
/* FP:symbol.rs-0443 */         allow,
/* FP:symbol.rs-0444 */         allow_fail,
/* FP:symbol.rs-0445 */         allow_internal_unsafe,
/* FP:symbol.rs-0446 */         allow_internal_unstable,
/* FP:symbol.rs-0447 */         altivec,
/* FP:symbol.rs-0448 */         alu32,
/* FP:symbol.rs-0449 */         always,
/* FP:symbol.rs-0450 */         analysis,
/* FP:symbol.rs-0451 */         and,
/* FP:symbol.rs-0452 */         and_then,
/* FP:symbol.rs-0453 */         anon,
/* FP:symbol.rs-0454 */         anon_adt,
/* FP:symbol.rs-0455 */         anon_assoc,
/* FP:symbol.rs-0456 */         anonymous_lifetime_in_impl_trait,
/* FP:symbol.rs-0457 */         any,
/* FP:symbol.rs-0458 */         append_const_msg,
/* FP:symbol.rs-0459 */         apx_target_feature,
/* FP:symbol.rs-0460 */         arbitrary_enum_discriminant,
/* FP:symbol.rs-0461 */         arbitrary_self_types,
/* FP:symbol.rs-0462 */         arbitrary_self_types_pointers,
/* FP:symbol.rs-0463 */         areg,
/* FP:symbol.rs-0464 */         args,
/* FP:symbol.rs-0465 */         arith_offset,
/* FP:symbol.rs-0466 */         arm,
/* FP:symbol.rs-0467 */         arm_target_feature,
/* FP:symbol.rs-0468 */         array,
/* FP:symbol.rs-0469 */         as_dash_needed: "as-needed",
/* FP:symbol.rs-0470 */         as_ptr,
/* FP:symbol.rs-0471 */         as_ref,
/* FP:symbol.rs-0472 */         as_str,
/* FP:symbol.rs-0473 */         asm,
/* FP:symbol.rs-0474 */         asm_cfg,
/* FP:symbol.rs-0475 */         asm_const,
/* FP:symbol.rs-0476 */         asm_experimental_arch,
/* FP:symbol.rs-0477 */         asm_experimental_reg,
/* FP:symbol.rs-0478 */         asm_goto,
/* FP:symbol.rs-0479 */         asm_goto_with_outputs,
/* FP:symbol.rs-0480 */         asm_sym,
/* FP:symbol.rs-0481 */         asm_unwind,
/* FP:symbol.rs-0482 */         assert,
/* FP:symbol.rs-0483 */         assert_eq,
/* FP:symbol.rs-0484 */         assert_eq_macro,
/* FP:symbol.rs-0485 */         assert_inhabited,
/* FP:symbol.rs-0486 */         assert_macro,
/* FP:symbol.rs-0487 */         assert_mem_uninitialized_valid,
/* FP:symbol.rs-0488 */         assert_ne_macro,
/* FP:symbol.rs-0489 */         assert_receiver_is_total_eq,
/* FP:symbol.rs-0490 */         assert_zero_valid,
/* FP:symbol.rs-0491 */         asserting,
/* FP:symbol.rs-0492 */         associated_const_equality,
/* FP:symbol.rs-0493 */         associated_consts,
/* FP:symbol.rs-0494 */         associated_type_bounds,
/* FP:symbol.rs-0495 */         associated_type_defaults,
/* FP:symbol.rs-0496 */         associated_types,
/* FP:symbol.rs-0497 */         assume,
/* FP:symbol.rs-0498 */         assume_init,
/* FP:symbol.rs-0499 */         asterisk: "*",
/* FP:symbol.rs-0500 */         async_await,
/* FP:symbol.rs-0501 */         async_call,
/* FP:symbol.rs-0502 */         async_call_mut,
/* FP:symbol.rs-0503 */         async_call_once,
/* FP:symbol.rs-0504 */         async_closure,
/* FP:symbol.rs-0505 */         async_drop,
/* FP:symbol.rs-0506 */         async_drop_in_place,
/* FP:symbol.rs-0507 */         async_fn,
/* FP:symbol.rs-0508 */         async_fn_in_dyn_trait,
/* FP:symbol.rs-0509 */         async_fn_in_trait,
/* FP:symbol.rs-0510 */         async_fn_kind_helper,
/* FP:symbol.rs-0511 */         async_fn_kind_upvars,
/* FP:symbol.rs-0512 */         async_fn_mut,
/* FP:symbol.rs-0513 */         async_fn_once,
/* FP:symbol.rs-0514 */         async_fn_once_output,
/* FP:symbol.rs-0515 */         async_fn_track_caller,
/* FP:symbol.rs-0516 */         async_fn_traits,
/* FP:symbol.rs-0517 */         async_for_loop,
/* FP:symbol.rs-0518 */         async_iterator,
/* FP:symbol.rs-0519 */         async_iterator_poll_next,
/* FP:symbol.rs-0520 */         async_trait_bounds,
/* FP:symbol.rs-0521 */         atomic,
/* FP:symbol.rs-0522 */         atomic_and,
/* FP:symbol.rs-0523 */         atomic_cxchg,
/* FP:symbol.rs-0524 */         atomic_cxchgweak,
/* FP:symbol.rs-0525 */         atomic_fence,
/* FP:symbol.rs-0526 */         atomic_load,
/* FP:symbol.rs-0527 */         atomic_max,
/* FP:symbol.rs-0528 */         atomic_min,
/* FP:symbol.rs-0529 */         atomic_mod,
/* FP:symbol.rs-0530 */         atomic_nand,
/* FP:symbol.rs-0531 */         atomic_or,
/* FP:symbol.rs-0532 */         atomic_singlethreadfence,
/* FP:symbol.rs-0533 */         atomic_store,
/* FP:symbol.rs-0534 */         atomic_umax,
/* FP:symbol.rs-0535 */         atomic_umin,
/* FP:symbol.rs-0536 */         atomic_xadd,
/* FP:symbol.rs-0537 */         atomic_xchg,
/* FP:symbol.rs-0538 */         atomic_xor,
/* FP:symbol.rs-0539 */         atomic_xsub,
/* FP:symbol.rs-0540 */         atomics,
/* FP:symbol.rs-0541 */         att_syntax,
/* FP:symbol.rs-0542 */         attr,
/* FP:symbol.rs-0543 */         attr_literals,
/* FP:symbol.rs-0544 */         attribute,
/* FP:symbol.rs-0545 */         attributes,
/* FP:symbol.rs-0546 */         audit_that,
/* FP:symbol.rs-0547 */         augmented_assignments,
/* FP:symbol.rs-0548 */         auto_traits,
/* FP:symbol.rs-0549 */         autodiff,
/* FP:symbol.rs-0550 */         autodiff_forward,
/* FP:symbol.rs-0551 */         autodiff_reverse,
/* FP:symbol.rs-0552 */         automatically_derived,
/* FP:symbol.rs-0553 */         available_externally,
/* FP:symbol.rs-0554 */         avx,
/* FP:symbol.rs-0555 */         avx10_target_feature,
/* FP:symbol.rs-0556 */         avx512_target_feature,
/* FP:symbol.rs-0557 */         avx512bw,
/* FP:symbol.rs-0558 */         avx512f,
/* FP:symbol.rs-0559 */         await_macro,
/* FP:symbol.rs-0560 */         bang,
/* FP:symbol.rs-0561 */         begin_panic,
/* FP:symbol.rs-0562 */         bench,
/* FP:symbol.rs-0563 */         bevy_ecs,
/* FP:symbol.rs-0564 */         bikeshed_guaranteed_no_drop,
/* FP:symbol.rs-0565 */         bin,
/* FP:symbol.rs-0566 */         binaryheap_iter,
/* FP:symbol.rs-0567 */         bind_by_move_pattern_guards,
/* FP:symbol.rs-0568 */         bindings_after_at,
/* FP:symbol.rs-0569 */         bitand,
/* FP:symbol.rs-0570 */         bitand_assign,
/* FP:symbol.rs-0571 */         bitor,
/* FP:symbol.rs-0572 */         bitor_assign,
/* FP:symbol.rs-0573 */         bitreverse,
/* FP:symbol.rs-0574 */         bitxor,
/* FP:symbol.rs-0575 */         bitxor_assign,
/* FP:symbol.rs-0576 */         black_box,
/* FP:symbol.rs-0577 */         block,
/* FP:symbol.rs-0578 */         bool,
/* FP:symbol.rs-0579 */         bool_then,
/* FP:symbol.rs-0580 */         borrowck_graphviz_format,
/* FP:symbol.rs-0581 */         borrowck_graphviz_postflow,
/* FP:symbol.rs-0582 */         box_new,
/* FP:symbol.rs-0583 */         box_patterns,
/* FP:symbol.rs-0584 */         box_syntax,
/* FP:symbol.rs-0585 */         boxed_slice,
/* FP:symbol.rs-0586 */         bpf_target_feature,
/* FP:symbol.rs-0587 */         braced_empty_structs,
/* FP:symbol.rs-0588 */         branch,
/* FP:symbol.rs-0589 */         breakpoint,
/* FP:symbol.rs-0590 */         bridge,
/* FP:symbol.rs-0591 */         bswap,
/* FP:symbol.rs-0592 */         btreemap_contains_key,
/* FP:symbol.rs-0593 */         btreemap_insert,
/* FP:symbol.rs-0594 */         btreeset_iter,
/* FP:symbol.rs-0595 */         built,
/* FP:symbol.rs-0596 */         builtin_syntax,
/* FP:symbol.rs-0597 */         bundle,
/* FP:symbol.rs-0598 */         c,
/* FP:symbol.rs-0599 */         c_dash_variadic,
/* FP:symbol.rs-0600 */         c_str,
/* FP:symbol.rs-0601 */         c_str_literals,
/* FP:symbol.rs-0602 */         c_unwind,
/* FP:symbol.rs-0603 */         c_variadic,
/* FP:symbol.rs-0604 */         c_void,
/* FP:symbol.rs-0605 */         call,
/* FP:symbol.rs-0606 */         call_mut,
/* FP:symbol.rs-0607 */         call_once,
/* FP:symbol.rs-0608 */         call_once_future,
/* FP:symbol.rs-0609 */         call_ref_future,
/* FP:symbol.rs-0610 */         caller_location,
/* FP:symbol.rs-0611 */         capture_disjoint_fields,
/* FP:symbol.rs-0612 */         carrying_mul_add,
/* FP:symbol.rs-0613 */         catch_unwind,
/* FP:symbol.rs-0614 */         cause,
/* FP:symbol.rs-0615 */         cdylib,
/* FP:symbol.rs-0616 */         ceilf16,
/* FP:symbol.rs-0617 */         ceilf32,
/* FP:symbol.rs-0618 */         ceilf64,
/* FP:symbol.rs-0619 */         ceilf128,
/* FP:symbol.rs-0620 */         cfg,
/* FP:symbol.rs-0621 */         cfg_accessible,
/* FP:symbol.rs-0622 */         cfg_attr,
/* FP:symbol.rs-0623 */         cfg_attr_multi,
/* FP:symbol.rs-0624 */         cfg_attr_trace: "<cfg_attr>", // must not be a valid identifier
/* FP:symbol.rs-0625 */         cfg_boolean_literals,
/* FP:symbol.rs-0626 */         cfg_contract_checks,
/* FP:symbol.rs-0627 */         cfg_doctest,
/* FP:symbol.rs-0628 */         cfg_emscripten_wasm_eh,
/* FP:symbol.rs-0629 */         cfg_eval,
/* FP:symbol.rs-0630 */         cfg_fmt_debug,
/* FP:symbol.rs-0631 */         cfg_hide,
/* FP:symbol.rs-0632 */         cfg_overflow_checks,
/* FP:symbol.rs-0633 */         cfg_panic,
/* FP:symbol.rs-0634 */         cfg_relocation_model,
/* FP:symbol.rs-0635 */         cfg_sanitize,
/* FP:symbol.rs-0636 */         cfg_sanitizer_cfi,
/* FP:symbol.rs-0637 */         cfg_select,
/* FP:symbol.rs-0638 */         cfg_target_abi,
/* FP:symbol.rs-0639 */         cfg_target_compact,
/* FP:symbol.rs-0640 */         cfg_target_feature,
/* FP:symbol.rs-0641 */         cfg_target_has_atomic,
/* FP:symbol.rs-0642 */         cfg_target_has_atomic_equal_alignment,
/* FP:symbol.rs-0643 */         cfg_target_has_reliable_f16_f128,
/* FP:symbol.rs-0644 */         cfg_target_thread_local,
/* FP:symbol.rs-0645 */         cfg_target_vendor,
/* FP:symbol.rs-0646 */         cfg_trace: "<cfg>", // must not be a valid identifier
/* FP:symbol.rs-0647 */         cfg_ub_checks,
/* FP:symbol.rs-0648 */         cfg_version,
/* FP:symbol.rs-0649 */         cfi,
/* FP:symbol.rs-0650 */         cfi_encoding,
/* FP:symbol.rs-0651 */         char,
/* FP:symbol.rs-0652 */         char_is_ascii,
/* FP:symbol.rs-0653 */         char_to_digit,
/* FP:symbol.rs-0654 */         child_id,
/* FP:symbol.rs-0655 */         child_kill,
/* FP:symbol.rs-0656 */         client,
/* FP:symbol.rs-0657 */         clippy,
/* FP:symbol.rs-0658 */         clobber_abi,
/* FP:symbol.rs-0659 */         clone,
/* FP:symbol.rs-0660 */         clone_closures,
/* FP:symbol.rs-0661 */         clone_fn,
/* FP:symbol.rs-0662 */         clone_from,
/* FP:symbol.rs-0663 */         closure,
/* FP:symbol.rs-0664 */         closure_lifetime_binder,
/* FP:symbol.rs-0665 */         closure_to_fn_coercion,
/* FP:symbol.rs-0666 */         closure_track_caller,
/* FP:symbol.rs-0667 */         cmp,
/* FP:symbol.rs-0668 */         cmp_max,
/* FP:symbol.rs-0669 */         cmp_min,
/* FP:symbol.rs-0670 */         cmp_ord_max,
/* FP:symbol.rs-0671 */         cmp_ord_min,
/* FP:symbol.rs-0672 */         cmp_partialeq_eq,
/* FP:symbol.rs-0673 */         cmp_partialeq_ne,
/* FP:symbol.rs-0674 */         cmp_partialord_cmp,
/* FP:symbol.rs-0675 */         cmp_partialord_ge,
/* FP:symbol.rs-0676 */         cmp_partialord_gt,
/* FP:symbol.rs-0677 */         cmp_partialord_le,
/* FP:symbol.rs-0678 */         cmp_partialord_lt,
/* FP:symbol.rs-0679 */         cmpxchg16b_target_feature,
/* FP:symbol.rs-0680 */         cmse_nonsecure_entry,
/* FP:symbol.rs-0681 */         coerce_pointee_validated,
/* FP:symbol.rs-0682 */         coerce_unsized,
/* FP:symbol.rs-0683 */         cold,
/* FP:symbol.rs-0684 */         cold_path,
/* FP:symbol.rs-0685 */         collapse_debuginfo,
/* FP:symbol.rs-0686 */         column,
/* FP:symbol.rs-0687 */         common,
/* FP:symbol.rs-0688 */         compare_bytes,
/* FP:symbol.rs-0689 */         compare_exchange,
/* FP:symbol.rs-0690 */         compare_exchange_weak,
/* FP:symbol.rs-0691 */         compile_error,
/* FP:symbol.rs-0692 */         compiler,
/* FP:symbol.rs-0693 */         compiler_builtins,
/* FP:symbol.rs-0694 */         compiler_fence,
/* FP:symbol.rs-0695 */         concat,
/* FP:symbol.rs-0696 */         concat_bytes,
/* FP:symbol.rs-0697 */         concat_idents,
/* FP:symbol.rs-0698 */         conservative_impl_trait,
/* FP:symbol.rs-0699 */         console,
/* FP:symbol.rs-0700 */         const_allocate,
/* FP:symbol.rs-0701 */         const_async_blocks,
/* FP:symbol.rs-0702 */         const_closures,
/* FP:symbol.rs-0703 */         const_compare_raw_pointers,
/* FP:symbol.rs-0704 */         const_constructor,
/* FP:symbol.rs-0705 */         const_continue,
/* FP:symbol.rs-0706 */         const_deallocate,
/* FP:symbol.rs-0707 */         const_destruct,
/* FP:symbol.rs-0708 */         const_eval_limit,
/* FP:symbol.rs-0709 */         const_eval_select,
/* FP:symbol.rs-0710 */         const_evaluatable_checked,
/* FP:symbol.rs-0711 */         const_extern_fn,
/* FP:symbol.rs-0712 */         const_fn,
/* FP:symbol.rs-0713 */         const_fn_floating_point_arithmetic,
/* FP:symbol.rs-0714 */         const_fn_fn_ptr_basics,
/* FP:symbol.rs-0715 */         const_fn_trait_bound,
/* FP:symbol.rs-0716 */         const_fn_transmute,
/* FP:symbol.rs-0717 */         const_fn_union,
/* FP:symbol.rs-0718 */         const_fn_unsize,
/* FP:symbol.rs-0719 */         const_for,
/* FP:symbol.rs-0720 */         const_format_args,
/* FP:symbol.rs-0721 */         const_generics,
/* FP:symbol.rs-0722 */         const_generics_defaults,
/* FP:symbol.rs-0723 */         const_if_match,
/* FP:symbol.rs-0724 */         const_impl_trait,
/* FP:symbol.rs-0725 */         const_in_array_repeat_expressions,
/* FP:symbol.rs-0726 */         const_indexing,
/* FP:symbol.rs-0727 */         const_let,
/* FP:symbol.rs-0728 */         const_loop,
/* FP:symbol.rs-0729 */         const_make_global,
/* FP:symbol.rs-0730 */         const_mut_refs,
/* FP:symbol.rs-0731 */         const_panic,
/* FP:symbol.rs-0732 */         const_panic_fmt,
/* FP:symbol.rs-0733 */         const_param_ty,
/* FP:symbol.rs-0734 */         const_precise_live_drops,
/* FP:symbol.rs-0735 */         const_ptr_cast,
/* FP:symbol.rs-0736 */         const_raw_ptr_deref,
/* FP:symbol.rs-0737 */         const_raw_ptr_to_usize_cast,
/* FP:symbol.rs-0738 */         const_refs_to_cell,
/* FP:symbol.rs-0739 */         const_refs_to_static,
/* FP:symbol.rs-0740 */         const_trait,
/* FP:symbol.rs-0741 */         const_trait_bound_opt_out,
/* FP:symbol.rs-0742 */         const_trait_impl,
/* FP:symbol.rs-0743 */         const_try,
/* FP:symbol.rs-0744 */         const_ty_placeholder: "<const_ty>",
/* FP:symbol.rs-0745 */         constant,
/* FP:symbol.rs-0746 */         constructor,
/* FP:symbol.rs-0747 */         contract_build_check_ensures,
/* FP:symbol.rs-0748 */         contract_check_ensures,
/* FP:symbol.rs-0749 */         contract_check_requires,
/* FP:symbol.rs-0750 */         contract_checks,
/* FP:symbol.rs-0751 */         contracts,
/* FP:symbol.rs-0752 */         contracts_ensures,
/* FP:symbol.rs-0753 */         contracts_internals,
/* FP:symbol.rs-0754 */         contracts_requires,
/* FP:symbol.rs-0755 */         convert,
/* FP:symbol.rs-0756 */         convert_identity,
/* FP:symbol.rs-0757 */         copy,
/* FP:symbol.rs-0758 */         copy_closures,
/* FP:symbol.rs-0759 */         copy_nonoverlapping,
/* FP:symbol.rs-0760 */         copysignf16,
/* FP:symbol.rs-0761 */         copysignf32,
/* FP:symbol.rs-0762 */         copysignf64,
/* FP:symbol.rs-0763 */         copysignf128,
/* FP:symbol.rs-0764 */         core,
/* FP:symbol.rs-0765 */         core_panic,
/* FP:symbol.rs-0766 */         core_panic_2015_macro,
/* FP:symbol.rs-0767 */         core_panic_2021_macro,
/* FP:symbol.rs-0768 */         core_panic_macro,
/* FP:symbol.rs-0769 */         coroutine,
/* FP:symbol.rs-0770 */         coroutine_clone,
/* FP:symbol.rs-0771 */         coroutine_resume,
/* FP:symbol.rs-0772 */         coroutine_return,
/* FP:symbol.rs-0773 */         coroutine_state,
/* FP:symbol.rs-0774 */         coroutine_yield,
/* FP:symbol.rs-0775 */         coroutines,
/* FP:symbol.rs-0776 */         cosf16,
/* FP:symbol.rs-0777 */         cosf32,
/* FP:symbol.rs-0778 */         cosf64,
/* FP:symbol.rs-0779 */         cosf128,
/* FP:symbol.rs-0780 */         count,
/* FP:symbol.rs-0781 */         coverage,
/* FP:symbol.rs-0782 */         coverage_attribute,
/* FP:symbol.rs-0783 */         cr,
/* FP:symbol.rs-0784 */         crate_in_paths,
/* FP:symbol.rs-0785 */         crate_local,
/* FP:symbol.rs-0786 */         crate_name,
/* FP:symbol.rs-0787 */         crate_type,
/* FP:symbol.rs-0788 */         crate_visibility_modifier,
/* FP:symbol.rs-0789 */         crt_dash_static: "crt-static",
/* FP:symbol.rs-0790 */         csky_target_feature,
/* FP:symbol.rs-0791 */         cstr_type,
/* FP:symbol.rs-0792 */         cstring_as_c_str,
/* FP:symbol.rs-0793 */         cstring_type,
/* FP:symbol.rs-0794 */         ctlz,
/* FP:symbol.rs-0795 */         ctlz_nonzero,
/* FP:symbol.rs-0796 */         ctpop,
/* FP:symbol.rs-0797 */         cttz,
/* FP:symbol.rs-0798 */         cttz_nonzero,
/* FP:symbol.rs-0799 */         custom_attribute,
/* FP:symbol.rs-0800 */         custom_code_classes_in_docs,
/* FP:symbol.rs-0801 */         custom_derive,
/* FP:symbol.rs-0802 */         custom_inner_attributes,
/* FP:symbol.rs-0803 */         custom_mir,
/* FP:symbol.rs-0804 */         custom_test_frameworks,
/* FP:symbol.rs-0805 */         d,
/* FP:symbol.rs-0806 */         d32,
/* FP:symbol.rs-0807 */         dbg_macro,
/* FP:symbol.rs-0808 */         dead_code,
/* FP:symbol.rs-0809 */         dealloc,
/* FP:symbol.rs-0810 */         debug,
/* FP:symbol.rs-0811 */         debug_assert_eq_macro,
/* FP:symbol.rs-0812 */         debug_assert_macro,
/* FP:symbol.rs-0813 */         debug_assert_ne_macro,
/* FP:symbol.rs-0814 */         debug_assertions,
/* FP:symbol.rs-0815 */         debug_struct,
/* FP:symbol.rs-0816 */         debug_struct_fields_finish,
/* FP:symbol.rs-0817 */         debug_tuple,
/* FP:symbol.rs-0818 */         debug_tuple_fields_finish,
/* FP:symbol.rs-0819 */         debugger_visualizer,
/* FP:symbol.rs-0820 */         decl_macro,
/* FP:symbol.rs-0821 */         declare_lint_pass,
/* FP:symbol.rs-0822 */         decode,
/* FP:symbol.rs-0823 */         decorated,
/* FP:symbol.rs-0824 */         default_alloc_error_handler,
/* FP:symbol.rs-0825 */         default_field_values,
/* FP:symbol.rs-0826 */         default_fn,
/* FP:symbol.rs-0827 */         default_lib_allocator,
/* FP:symbol.rs-0828 */         default_method_body_is_const,
/* FP:symbol.rs-0829 */         // --------------------------
/* FP:symbol.rs-0830 */         // Lang items which are used only for experiments with auto traits with default bounds.
/* FP:symbol.rs-0831 */         // These lang items are not actually defined in core/std. Experiment is a part of
/* FP:symbol.rs-0832 */         // `MCP: Low level components for async drop`(https://github.com/rust-lang/compiler-team/issues/727)
/* FP:symbol.rs-0833 */         default_trait1,
/* FP:symbol.rs-0834 */         default_trait2,
/* FP:symbol.rs-0835 */         default_trait3,
/* FP:symbol.rs-0836 */         default_trait4,
/* FP:symbol.rs-0837 */         // --------------------------
/* FP:symbol.rs-0838 */         default_type_parameter_fallback,
/* FP:symbol.rs-0839 */         default_type_params,
/* FP:symbol.rs-0840 */         define_opaque,
/* FP:symbol.rs-0841 */         delayed_bug_from_inside_query,
/* FP:symbol.rs-0842 */         deny,
/* FP:symbol.rs-0843 */         deprecated,
/* FP:symbol.rs-0844 */         deprecated_safe,
/* FP:symbol.rs-0845 */         deprecated_suggestion,
/* FP:symbol.rs-0846 */         deref,
/* FP:symbol.rs-0847 */         deref_method,
/* FP:symbol.rs-0848 */         deref_mut,
/* FP:symbol.rs-0849 */         deref_mut_method,
/* FP:symbol.rs-0850 */         deref_patterns,
/* FP:symbol.rs-0851 */         deref_pure,
/* FP:symbol.rs-0852 */         deref_target,
/* FP:symbol.rs-0853 */         derive,
/* FP:symbol.rs-0854 */         derive_coerce_pointee,
/* FP:symbol.rs-0855 */         derive_const,
/* FP:symbol.rs-0856 */         derive_const_issue: "118304",
/* FP:symbol.rs-0857 */         derive_default_enum,
/* FP:symbol.rs-0858 */         derive_from,
/* FP:symbol.rs-0859 */         derive_smart_pointer,
/* FP:symbol.rs-0860 */         destruct,
/* FP:symbol.rs-0861 */         destructuring_assignment,
/* FP:symbol.rs-0862 */         diagnostic,
/* FP:symbol.rs-0863 */         diagnostic_namespace,
/* FP:symbol.rs-0864 */         dialect,
/* FP:symbol.rs-0865 */         direct,
/* FP:symbol.rs-0866 */         discriminant_kind,
/* FP:symbol.rs-0867 */         discriminant_type,
/* FP:symbol.rs-0868 */         discriminant_value,
/* FP:symbol.rs-0869 */         disjoint_bitor,
/* FP:symbol.rs-0870 */         dispatch_from_dyn,
/* FP:symbol.rs-0871 */         div,
/* FP:symbol.rs-0872 */         div_assign,
/* FP:symbol.rs-0873 */         diverging_block_default,
/* FP:symbol.rs-0874 */         dl,
/* FP:symbol.rs-0875 */         do_not_recommend,
/* FP:symbol.rs-0876 */         doc,
/* FP:symbol.rs-0877 */         doc_alias,
/* FP:symbol.rs-0878 */         doc_auto_cfg,
/* FP:symbol.rs-0879 */         doc_cfg,
/* FP:symbol.rs-0880 */         doc_cfg_hide,
/* FP:symbol.rs-0881 */         doc_keyword,
/* FP:symbol.rs-0882 */         doc_masked,
/* FP:symbol.rs-0883 */         doc_notable_trait,
/* FP:symbol.rs-0884 */         doc_primitive,
/* FP:symbol.rs-0885 */         doc_spotlight,
/* FP:symbol.rs-0886 */         doctest,
/* FP:symbol.rs-0887 */         document_private_items,
/* FP:symbol.rs-0888 */         dotdot: "..",
/* FP:symbol.rs-0889 */         dotdot_in_tuple_patterns,
/* FP:symbol.rs-0890 */         dotdoteq_in_patterns,
/* FP:symbol.rs-0891 */         dreg,
/* FP:symbol.rs-0892 */         dreg_low8,
/* FP:symbol.rs-0893 */         dreg_low16,
/* FP:symbol.rs-0894 */         drop,
/* FP:symbol.rs-0895 */         drop_in_place,
/* FP:symbol.rs-0896 */         drop_types_in_const,
/* FP:symbol.rs-0897 */         dropck_eyepatch,
/* FP:symbol.rs-0898 */         dropck_parametricity,
/* FP:symbol.rs-0899 */         dummy: "<!dummy!>", // use this instead of `sym::empty` for symbols that won't be used
/* FP:symbol.rs-0900 */         dummy_cgu_name,
/* FP:symbol.rs-0901 */         dylib,
/* FP:symbol.rs-0902 */         dyn_compatible_for_dispatch,
/* FP:symbol.rs-0903 */         dyn_metadata,
/* FP:symbol.rs-0904 */         dyn_star,
/* FP:symbol.rs-0905 */         dyn_trait,
/* FP:symbol.rs-0906 */         dynamic_no_pic: "dynamic-no-pic",
/* FP:symbol.rs-0907 */         e,
/* FP:symbol.rs-0908 */         edition_panic,
/* FP:symbol.rs-0909 */         effective_target_features,
/* FP:symbol.rs-0910 */         effects,
/* FP:symbol.rs-0911 */         eh_catch_typeinfo,
/* FP:symbol.rs-0912 */         eh_personality,
/* FP:symbol.rs-0913 */         emit,
/* FP:symbol.rs-0914 */         emit_enum,
/* FP:symbol.rs-0915 */         emit_enum_variant,
/* FP:symbol.rs-0916 */         emit_enum_variant_arg,
/* FP:symbol.rs-0917 */         emit_struct,
/* FP:symbol.rs-0918 */         emit_struct_field,
/* FP:symbol.rs-0919 */         // Notes about `sym::empty`:
/* FP:symbol.rs-0920 */         // - It should only be used when it genuinely means "empty symbol". Use
/* FP:symbol.rs-0921 */         //   `Option<Symbol>` when "no symbol" is a possibility.
/* FP:symbol.rs-0922 */         // - For dummy symbols that are never used and absolutely must be
/* FP:symbol.rs-0923 */         //   present, it's better to use `sym::dummy` than `sym::empty`, because
/* FP:symbol.rs-0924 */         //   it's clearer that it's intended as a dummy value, and more likely
/* FP:symbol.rs-0925 */         //   to be detected if it accidentally does get used.
/* FP:symbol.rs-0926 */         empty: "",
/* FP:symbol.rs-0927 */         emscripten_wasm_eh,
/* FP:symbol.rs-0928 */         enable,
/* FP:symbol.rs-0929 */         encode,
/* FP:symbol.rs-0930 */         end,
/* FP:symbol.rs-0931 */         entry_nops,
/* FP:symbol.rs-0932 */         enumerate_method,
/* FP:symbol.rs-0933 */         env,
/* FP:symbol.rs-0934 */         env_CFG_RELEASE: env!("CFG_RELEASE"),
/* FP:symbol.rs-0935 */         eprint_macro,
/* FP:symbol.rs-0936 */         eprintln_macro,
/* FP:symbol.rs-0937 */         eq,
/* FP:symbol.rs-0938 */         ergonomic_clones,
/* FP:symbol.rs-0939 */         ermsb_target_feature,
/* FP:symbol.rs-0940 */         exact_div,
/* FP:symbol.rs-0941 */         except,
/* FP:symbol.rs-0942 */         exchange_malloc,
/* FP:symbol.rs-0943 */         exclusive_range_pattern,
/* FP:symbol.rs-0944 */         exhaustive_integer_patterns,
/* FP:symbol.rs-0945 */         exhaustive_patterns,
/* FP:symbol.rs-0946 */         existential_type,
/* FP:symbol.rs-0947 */         exp2f16,
/* FP:symbol.rs-0948 */         exp2f32,
/* FP:symbol.rs-0949 */         exp2f64,
/* FP:symbol.rs-0950 */         exp2f128,
/* FP:symbol.rs-0951 */         expect,
/* FP:symbol.rs-0952 */         expected,
/* FP:symbol.rs-0953 */         expf16,
/* FP:symbol.rs-0954 */         expf32,
/* FP:symbol.rs-0955 */         expf64,
/* FP:symbol.rs-0956 */         expf128,
/* FP:symbol.rs-0957 */         explicit_extern_abis,
/* FP:symbol.rs-0958 */         explicit_generic_args_with_impl_trait,
/* FP:symbol.rs-0959 */         explicit_tail_calls,
/* FP:symbol.rs-0960 */         export_name,
/* FP:symbol.rs-0961 */         export_stable,
/* FP:symbol.rs-0962 */         expr,
/* FP:symbol.rs-0963 */         expr_2021,
/* FP:symbol.rs-0964 */         expr_fragment_specifier_2024,
/* FP:symbol.rs-0965 */         extended_key_value_attributes,
/* FP:symbol.rs-0966 */         extended_varargs_abi_support,
/* FP:symbol.rs-0967 */         extern_absolute_paths,
/* FP:symbol.rs-0968 */         extern_crate_item_prelude,
/* FP:symbol.rs-0969 */         extern_crate_self,
/* FP:symbol.rs-0970 */         extern_in_paths,
/* FP:symbol.rs-0971 */         extern_prelude,
/* FP:symbol.rs-0972 */         extern_system_varargs,
/* FP:symbol.rs-0973 */         extern_types,
/* FP:symbol.rs-0974 */         extern_weak,
/* FP:symbol.rs-0975 */         external,
/* FP:symbol.rs-0976 */         external_doc,
/* FP:symbol.rs-0977 */         f,
/* FP:symbol.rs-0978 */         f16,
/* FP:symbol.rs-0979 */         f16_epsilon,
/* FP:symbol.rs-0980 */         f16_nan,
/* FP:symbol.rs-0981 */         f16c_target_feature,
/* FP:symbol.rs-0982 */         f32,
/* FP:symbol.rs-0983 */         f32_epsilon,
/* FP:symbol.rs-0984 */         f32_legacy_const_digits,
/* FP:symbol.rs-0985 */         f32_legacy_const_epsilon,
/* FP:symbol.rs-0986 */         f32_legacy_const_infinity,
/* FP:symbol.rs-0987 */         f32_legacy_const_mantissa_dig,
/* FP:symbol.rs-0988 */         f32_legacy_const_max,
/* FP:symbol.rs-0989 */         f32_legacy_const_max_10_exp,
/* FP:symbol.rs-0990 */         f32_legacy_const_max_exp,
/* FP:symbol.rs-0991 */         f32_legacy_const_min,
/* FP:symbol.rs-0992 */         f32_legacy_const_min_10_exp,
/* FP:symbol.rs-0993 */         f32_legacy_const_min_exp,
/* FP:symbol.rs-0994 */         f32_legacy_const_min_positive,
/* FP:symbol.rs-0995 */         f32_legacy_const_nan,
/* FP:symbol.rs-0996 */         f32_legacy_const_neg_infinity,
/* FP:symbol.rs-0997 */         f32_legacy_const_radix,
/* FP:symbol.rs-0998 */         f32_nan,
/* FP:symbol.rs-0999 */         f64,
/* FP:symbol.rs-1000 */         f64_epsilon,
/* FP:symbol.rs-1001 */         f64_legacy_const_digits,
/* FP:symbol.rs-1002 */         f64_legacy_const_epsilon,
/* FP:symbol.rs-1003 */         f64_legacy_const_infinity,
/* FP:symbol.rs-1004 */         f64_legacy_const_mantissa_dig,
/* FP:symbol.rs-1005 */         f64_legacy_const_max,
/* FP:symbol.rs-1006 */         f64_legacy_const_max_10_exp,
/* FP:symbol.rs-1007 */         f64_legacy_const_max_exp,
/* FP:symbol.rs-1008 */         f64_legacy_const_min,
/* FP:symbol.rs-1009 */         f64_legacy_const_min_10_exp,
/* FP:symbol.rs-1010 */         f64_legacy_const_min_exp,
/* FP:symbol.rs-1011 */         f64_legacy_const_min_positive,
/* FP:symbol.rs-1012 */         f64_legacy_const_nan,
/* FP:symbol.rs-1013 */         f64_legacy_const_neg_infinity,
/* FP:symbol.rs-1014 */         f64_legacy_const_radix,
/* FP:symbol.rs-1015 */         f64_nan,
/* FP:symbol.rs-1016 */         f128,
/* FP:symbol.rs-1017 */         f128_epsilon,
/* FP:symbol.rs-1018 */         f128_nan,
/* FP:symbol.rs-1019 */         fabsf16,
/* FP:symbol.rs-1020 */         fabsf32,
/* FP:symbol.rs-1021 */         fabsf64,
/* FP:symbol.rs-1022 */         fabsf128,
/* FP:symbol.rs-1023 */         fadd_algebraic,
/* FP:symbol.rs-1024 */         fadd_fast,
/* FP:symbol.rs-1025 */         fake_variadic,
/* FP:symbol.rs-1026 */         fallback,
/* FP:symbol.rs-1027 */         fdiv_algebraic,
/* FP:symbol.rs-1028 */         fdiv_fast,
/* FP:symbol.rs-1029 */         feature,
/* FP:symbol.rs-1030 */         fence,
/* FP:symbol.rs-1031 */         ferris: "🦀",
/* FP:symbol.rs-1032 */         fetch_update,
/* FP:symbol.rs-1033 */         ffi,
/* FP:symbol.rs-1034 */         ffi_const,
/* FP:symbol.rs-1035 */         ffi_pure,
/* FP:symbol.rs-1036 */         ffi_returns_twice,
/* FP:symbol.rs-1037 */         field,
/* FP:symbol.rs-1038 */         field_init_shorthand,
/* FP:symbol.rs-1039 */         file,
/* FP:symbol.rs-1040 */         file_options,
/* FP:symbol.rs-1041 */         flags,
/* FP:symbol.rs-1042 */         float,
/* FP:symbol.rs-1043 */         float_to_int_unchecked,
/* FP:symbol.rs-1044 */         floorf16,
/* FP:symbol.rs-1045 */         floorf32,
/* FP:symbol.rs-1046 */         floorf64,
/* FP:symbol.rs-1047 */         floorf128,
/* FP:symbol.rs-1048 */         fmaf16,
/* FP:symbol.rs-1049 */         fmaf32,
/* FP:symbol.rs-1050 */         fmaf64,
/* FP:symbol.rs-1051 */         fmaf128,
/* FP:symbol.rs-1052 */         fmt,
/* FP:symbol.rs-1053 */         fmt_debug,
/* FP:symbol.rs-1054 */         fmul_algebraic,
/* FP:symbol.rs-1055 */         fmul_fast,
/* FP:symbol.rs-1056 */         fmuladdf16,
/* FP:symbol.rs-1057 */         fmuladdf32,
/* FP:symbol.rs-1058 */         fmuladdf64,
/* FP:symbol.rs-1059 */         fmuladdf128,
/* FP:symbol.rs-1060 */         fn_align,
/* FP:symbol.rs-1061 */         fn_body,
/* FP:symbol.rs-1062 */         fn_delegation,
/* FP:symbol.rs-1063 */         fn_must_use,
/* FP:symbol.rs-1064 */         fn_mut,
/* FP:symbol.rs-1065 */         fn_once,
/* FP:symbol.rs-1066 */         fn_once_output,
/* FP:symbol.rs-1067 */         fn_ptr_addr,
/* FP:symbol.rs-1068 */         fn_ptr_trait,
/* FP:symbol.rs-1069 */         forbid,
/* FP:symbol.rs-1070 */         force_target_feature,
/* FP:symbol.rs-1071 */         forget,
/* FP:symbol.rs-1072 */         format,
/* FP:symbol.rs-1073 */         format_args,
/* FP:symbol.rs-1074 */         format_args_capture,
/* FP:symbol.rs-1075 */         format_args_macro,
/* FP:symbol.rs-1076 */         format_args_nl,
/* FP:symbol.rs-1077 */         format_argument,
/* FP:symbol.rs-1078 */         format_arguments,
/* FP:symbol.rs-1079 */         format_count,
/* FP:symbol.rs-1080 */         format_macro,
/* FP:symbol.rs-1081 */         format_placeholder,
/* FP:symbol.rs-1082 */         format_unsafe_arg,
/* FP:symbol.rs-1083 */         framework,
/* FP:symbol.rs-1084 */         freeze,
/* FP:symbol.rs-1085 */         freeze_impls,
/* FP:symbol.rs-1086 */         freg,
/* FP:symbol.rs-1087 */         frem_algebraic,
/* FP:symbol.rs-1088 */         frem_fast,
/* FP:symbol.rs-1089 */         from,
/* FP:symbol.rs-1090 */         from_desugaring,
/* FP:symbol.rs-1091 */         from_fn,
/* FP:symbol.rs-1092 */         from_iter,
/* FP:symbol.rs-1093 */         from_iter_fn,
/* FP:symbol.rs-1094 */         from_output,
/* FP:symbol.rs-1095 */         from_residual,
/* FP:symbol.rs-1096 */         from_size_align_unchecked,
/* FP:symbol.rs-1097 */         from_str_method,
/* FP:symbol.rs-1098 */         from_u16,
/* FP:symbol.rs-1099 */         from_usize,
/* FP:symbol.rs-1100 */         from_yeet,
/* FP:symbol.rs-1101 */         frontmatter,
/* FP:symbol.rs-1102 */         fs_create_dir,
/* FP:symbol.rs-1103 */         fsub_algebraic,
/* FP:symbol.rs-1104 */         fsub_fast,
/* FP:symbol.rs-1105 */         full,
/* FP:symbol.rs-1106 */         fundamental,
/* FP:symbol.rs-1107 */         fused_iterator,
/* FP:symbol.rs-1108 */         future,
/* FP:symbol.rs-1109 */         future_drop_poll,
/* FP:symbol.rs-1110 */         future_output,
/* FP:symbol.rs-1111 */         future_trait,
/* FP:symbol.rs-1112 */         fxsr,
/* FP:symbol.rs-1113 */         gdb_script_file,
/* FP:symbol.rs-1114 */         ge,
/* FP:symbol.rs-1115 */         gen_blocks,
/* FP:symbol.rs-1116 */         gen_future,
/* FP:symbol.rs-1117 */         generator_clone,
/* FP:symbol.rs-1118 */         generators,
/* FP:symbol.rs-1119 */         generic_arg_infer,
/* FP:symbol.rs-1120 */         generic_assert,
/* FP:symbol.rs-1121 */         generic_associated_types,
/* FP:symbol.rs-1122 */         generic_associated_types_extended,
/* FP:symbol.rs-1123 */         generic_const_exprs,
/* FP:symbol.rs-1124 */         generic_const_items,
/* FP:symbol.rs-1125 */         generic_const_parameter_types,
/* FP:symbol.rs-1126 */         generic_param_attrs,
/* FP:symbol.rs-1127 */         generic_pattern_types,
/* FP:symbol.rs-1128 */         get_context,
/* FP:symbol.rs-1129 */         global_alloc_ty,
/* FP:symbol.rs-1130 */         global_allocator,
/* FP:symbol.rs-1131 */         global_asm,
/* FP:symbol.rs-1132 */         global_registration,
/* FP:symbol.rs-1133 */         globs,
/* FP:symbol.rs-1134 */         gt,
/* FP:symbol.rs-1135 */         guard_patterns,
/* FP:symbol.rs-1136 */         half_open_range_patterns,
/* FP:symbol.rs-1137 */         half_open_range_patterns_in_slices,
/* FP:symbol.rs-1138 */         hash,
/* FP:symbol.rs-1139 */         hashmap_contains_key,
/* FP:symbol.rs-1140 */         hashmap_drain_ty,
/* FP:symbol.rs-1141 */         hashmap_insert,
/* FP:symbol.rs-1142 */         hashmap_iter_mut_ty,
/* FP:symbol.rs-1143 */         hashmap_iter_ty,
/* FP:symbol.rs-1144 */         hashmap_keys_ty,
/* FP:symbol.rs-1145 */         hashmap_values_mut_ty,
/* FP:symbol.rs-1146 */         hashmap_values_ty,
/* FP:symbol.rs-1147 */         hashset_drain_ty,
/* FP:symbol.rs-1148 */         hashset_iter,
/* FP:symbol.rs-1149 */         hashset_iter_ty,
/* FP:symbol.rs-1150 */         hexagon_target_feature,
/* FP:symbol.rs-1151 */         hidden,
/* FP:symbol.rs-1152 */         hint,
/* FP:symbol.rs-1153 */         homogeneous_aggregate,
/* FP:symbol.rs-1154 */         host,
/* FP:symbol.rs-1155 */         html_favicon_url,
/* FP:symbol.rs-1156 */         html_logo_url,
/* FP:symbol.rs-1157 */         html_no_source,
/* FP:symbol.rs-1158 */         html_playground_url,
/* FP:symbol.rs-1159 */         html_root_url,
/* FP:symbol.rs-1160 */         hwaddress,
/* FP:symbol.rs-1161 */         i,
/* FP:symbol.rs-1162 */         i8,
/* FP:symbol.rs-1163 */         i8_legacy_const_max,
/* FP:symbol.rs-1164 */         i8_legacy_const_min,
/* FP:symbol.rs-1165 */         i8_legacy_fn_max_value,
/* FP:symbol.rs-1166 */         i8_legacy_fn_min_value,
/* FP:symbol.rs-1167 */         i8_legacy_mod,
/* FP:symbol.rs-1168 */         i16,
/* FP:symbol.rs-1169 */         i16_legacy_const_max,
/* FP:symbol.rs-1170 */         i16_legacy_const_min,
/* FP:symbol.rs-1171 */         i16_legacy_fn_max_value,
/* FP:symbol.rs-1172 */         i16_legacy_fn_min_value,
/* FP:symbol.rs-1173 */         i16_legacy_mod,
/* FP:symbol.rs-1174 */         i32,
/* FP:symbol.rs-1175 */         i32_legacy_const_max,
/* FP:symbol.rs-1176 */         i32_legacy_const_min,
/* FP:symbol.rs-1177 */         i32_legacy_fn_max_value,
/* FP:symbol.rs-1178 */         i32_legacy_fn_min_value,
/* FP:symbol.rs-1179 */         i32_legacy_mod,
/* FP:symbol.rs-1180 */         i64,
/* FP:symbol.rs-1181 */         i64_legacy_const_max,
/* FP:symbol.rs-1182 */         i64_legacy_const_min,
/* FP:symbol.rs-1183 */         i64_legacy_fn_max_value,
/* FP:symbol.rs-1184 */         i64_legacy_fn_min_value,
/* FP:symbol.rs-1185 */         i64_legacy_mod,
/* FP:symbol.rs-1186 */         i128,
/* FP:symbol.rs-1187 */         i128_legacy_const_max,
/* FP:symbol.rs-1188 */         i128_legacy_const_min,
/* FP:symbol.rs-1189 */         i128_legacy_fn_max_value,
/* FP:symbol.rs-1190 */         i128_legacy_fn_min_value,
/* FP:symbol.rs-1191 */         i128_legacy_mod,
/* FP:symbol.rs-1192 */         i128_type,
/* FP:symbol.rs-1193 */         ident,
/* FP:symbol.rs-1194 */         if_let,
/* FP:symbol.rs-1195 */         if_let_guard,
/* FP:symbol.rs-1196 */         if_let_rescope,
/* FP:symbol.rs-1197 */         if_while_or_patterns,
/* FP:symbol.rs-1198 */         ignore,
/* FP:symbol.rs-1199 */         impl_header_lifetime_elision,
/* FP:symbol.rs-1200 */         impl_lint_pass,
/* FP:symbol.rs-1201 */         impl_trait_in_assoc_type,
/* FP:symbol.rs-1202 */         impl_trait_in_bindings,
/* FP:symbol.rs-1203 */         impl_trait_in_fn_trait_return,
/* FP:symbol.rs-1204 */         impl_trait_projections,
/* FP:symbol.rs-1205 */         implement_via_object,
/* FP:symbol.rs-1206 */         implied_by,
/* FP:symbol.rs-1207 */         import,
/* FP:symbol.rs-1208 */         import_name_type,
/* FP:symbol.rs-1209 */         import_shadowing,
/* FP:symbol.rs-1210 */         import_trait_associated_functions,
/* FP:symbol.rs-1211 */         imported_main,
/* FP:symbol.rs-1212 */         in_band_lifetimes,
/* FP:symbol.rs-1213 */         include,
/* FP:symbol.rs-1214 */         include_bytes,
/* FP:symbol.rs-1215 */         include_bytes_macro,
/* FP:symbol.rs-1216 */         include_str,
/* FP:symbol.rs-1217 */         include_str_macro,
/* FP:symbol.rs-1218 */         inclusive_range_syntax,
/* FP:symbol.rs-1219 */         index,
/* FP:symbol.rs-1220 */         index_mut,
/* FP:symbol.rs-1221 */         infer_outlives_requirements,
/* FP:symbol.rs-1222 */         infer_static_outlives_requirements,
/* FP:symbol.rs-1223 */         inherent_associated_types,
/* FP:symbol.rs-1224 */         inherit,
/* FP:symbol.rs-1225 */         initial,
/* FP:symbol.rs-1226 */         inlateout,
/* FP:symbol.rs-1227 */         inline,
/* FP:symbol.rs-1228 */         inline_const,
/* FP:symbol.rs-1229 */         inline_const_pat,
/* FP:symbol.rs-1230 */         inout,
/* FP:symbol.rs-1231 */         instant_now,
/* FP:symbol.rs-1232 */         instruction_set,
/* FP:symbol.rs-1233 */         integer_: "integer", // underscore to avoid clashing with the function `sym::integer` below
/* FP:symbol.rs-1234 */         integral,
/* FP:symbol.rs-1235 */         internal,
/* FP:symbol.rs-1236 */         internal_features,
/* FP:symbol.rs-1237 */         into_async_iter_into_iter,
/* FP:symbol.rs-1238 */         into_future,
/* FP:symbol.rs-1239 */         into_iter,
/* FP:symbol.rs-1240 */         intra_doc_pointers,
/* FP:symbol.rs-1241 */         intrinsics,
/* FP:symbol.rs-1242 */         intrinsics_unaligned_volatile_load,
/* FP:symbol.rs-1243 */         intrinsics_unaligned_volatile_store,
/* FP:symbol.rs-1244 */         io_error_new,
/* FP:symbol.rs-1245 */         io_errorkind,
/* FP:symbol.rs-1246 */         io_stderr,
/* FP:symbol.rs-1247 */         io_stdout,
/* FP:symbol.rs-1248 */         irrefutable_let_patterns,
/* FP:symbol.rs-1249 */         is,
/* FP:symbol.rs-1250 */         is_val_statically_known,
/* FP:symbol.rs-1251 */         isa_attribute,
/* FP:symbol.rs-1252 */         isize,
/* FP:symbol.rs-1253 */         isize_legacy_const_max,
/* FP:symbol.rs-1254 */         isize_legacy_const_min,
/* FP:symbol.rs-1255 */         isize_legacy_fn_max_value,
/* FP:symbol.rs-1256 */         isize_legacy_fn_min_value,
/* FP:symbol.rs-1257 */         isize_legacy_mod,
/* FP:symbol.rs-1258 */         issue,
/* FP:symbol.rs-1259 */         issue_5723_bootstrap,
/* FP:symbol.rs-1260 */         issue_tracker_base_url,
/* FP:symbol.rs-1261 */         item,
/* FP:symbol.rs-1262 */         item_like_imports,
/* FP:symbol.rs-1263 */         iter,
/* FP:symbol.rs-1264 */         iter_cloned,
/* FP:symbol.rs-1265 */         iter_copied,
/* FP:symbol.rs-1266 */         iter_filter,
/* FP:symbol.rs-1267 */         iter_mut,
/* FP:symbol.rs-1268 */         iter_repeat,
/* FP:symbol.rs-1269 */         iterator,
/* FP:symbol.rs-1270 */         iterator_collect_fn,
/* FP:symbol.rs-1271 */         kcfi,
/* FP:symbol.rs-1272 */         kernel_address,
/* FP:symbol.rs-1273 */         keylocker_x86,
/* FP:symbol.rs-1274 */         keyword,
/* FP:symbol.rs-1275 */         kind,
/* FP:symbol.rs-1276 */         kreg,
/* FP:symbol.rs-1277 */         kreg0,
/* FP:symbol.rs-1278 */         label,
/* FP:symbol.rs-1279 */         label_break_value,
/* FP:symbol.rs-1280 */         lahfsahf_target_feature,
/* FP:symbol.rs-1281 */         lang,
/* FP:symbol.rs-1282 */         lang_items,
/* FP:symbol.rs-1283 */         large_assignments,
/* FP:symbol.rs-1284 */         last,
/* FP:symbol.rs-1285 */         lateout,
/* FP:symbol.rs-1286 */         lazy_normalization_consts,
/* FP:symbol.rs-1287 */         lazy_type_alias,
/* FP:symbol.rs-1288 */         le,
/* FP:symbol.rs-1289 */         legacy_receiver,
/* FP:symbol.rs-1290 */         len,
/* FP:symbol.rs-1291 */         let_chains,
/* FP:symbol.rs-1292 */         let_else,
/* FP:symbol.rs-1293 */         lhs,
/* FP:symbol.rs-1294 */         lib,
/* FP:symbol.rs-1295 */         libc,
/* FP:symbol.rs-1296 */         lifetime,
/* FP:symbol.rs-1297 */         lifetime_capture_rules_2024,
/* FP:symbol.rs-1298 */         lifetimes,
/* FP:symbol.rs-1299 */         likely,
/* FP:symbol.rs-1300 */         line,
/* FP:symbol.rs-1301 */         link,
/* FP:symbol.rs-1302 */         link_arg_attribute,
/* FP:symbol.rs-1303 */         link_args,
/* FP:symbol.rs-1304 */         link_cfg,
/* FP:symbol.rs-1305 */         link_dash_arg: "link-arg",
/* FP:symbol.rs-1306 */         link_llvm_intrinsics,
/* FP:symbol.rs-1307 */         link_name,
/* FP:symbol.rs-1308 */         link_ordinal,
/* FP:symbol.rs-1309 */         link_section,
/* FP:symbol.rs-1310 */         linkage,
/* FP:symbol.rs-1311 */         linker,
/* FP:symbol.rs-1312 */         linker_messages,
/* FP:symbol.rs-1313 */         linkonce,
/* FP:symbol.rs-1314 */         linkonce_odr,
/* FP:symbol.rs-1315 */         lint_reasons,
/* FP:symbol.rs-1316 */         literal,
/* FP:symbol.rs-1317 */         load,
/* FP:symbol.rs-1318 */         loaded_from_disk,
/* FP:symbol.rs-1319 */         local,
/* FP:symbol.rs-1320 */         local_inner_macros,
/* FP:symbol.rs-1321 */         log2f16,
/* FP:symbol.rs-1322 */         log2f32,
/* FP:symbol.rs-1323 */         log2f64,
/* FP:symbol.rs-1324 */         log2f128,
/* FP:symbol.rs-1325 */         log10f16,
/* FP:symbol.rs-1326 */         log10f32,
/* FP:symbol.rs-1327 */         log10f64,
/* FP:symbol.rs-1328 */         log10f128,
/* FP:symbol.rs-1329 */         log_syntax,
/* FP:symbol.rs-1330 */         logf16,
/* FP:symbol.rs-1331 */         logf32,
/* FP:symbol.rs-1332 */         logf64,
/* FP:symbol.rs-1333 */         logf128,
/* FP:symbol.rs-1334 */         loongarch_target_feature,
/* FP:symbol.rs-1335 */         loop_break_value,
/* FP:symbol.rs-1336 */         loop_match,
/* FP:symbol.rs-1337 */         lt,
/* FP:symbol.rs-1338 */         m68k_target_feature,
/* FP:symbol.rs-1339 */         macro_at_most_once_rep,
/* FP:symbol.rs-1340 */         macro_attr,
/* FP:symbol.rs-1341 */         macro_attributes_in_derive_output,
/* FP:symbol.rs-1342 */         macro_concat,
/* FP:symbol.rs-1343 */         macro_derive,
/* FP:symbol.rs-1344 */         macro_escape,
/* FP:symbol.rs-1345 */         macro_export,
/* FP:symbol.rs-1346 */         macro_lifetime_matcher,
/* FP:symbol.rs-1347 */         macro_literal_matcher,
/* FP:symbol.rs-1348 */         macro_metavar_expr,
/* FP:symbol.rs-1349 */         macro_metavar_expr_concat,
/* FP:symbol.rs-1350 */         macro_reexport,
/* FP:symbol.rs-1351 */         macro_use,
/* FP:symbol.rs-1352 */         macro_vis_matcher,
/* FP:symbol.rs-1353 */         macros_in_extern,
/* FP:symbol.rs-1354 */         main,
/* FP:symbol.rs-1355 */         managed_boxes,
/* FP:symbol.rs-1356 */         manually_drop,
/* FP:symbol.rs-1357 */         map,
/* FP:symbol.rs-1358 */         map_err,
/* FP:symbol.rs-1359 */         marker,
/* FP:symbol.rs-1360 */         marker_trait_attr,
/* FP:symbol.rs-1361 */         masked,
/* FP:symbol.rs-1362 */         match_beginning_vert,
/* FP:symbol.rs-1363 */         match_default_bindings,
/* FP:symbol.rs-1364 */         matches_macro,
/* FP:symbol.rs-1365 */         maximumf16,
/* FP:symbol.rs-1366 */         maximumf32,
/* FP:symbol.rs-1367 */         maximumf64,
/* FP:symbol.rs-1368 */         maximumf128,
/* FP:symbol.rs-1369 */         maxnumf16,
/* FP:symbol.rs-1370 */         maxnumf32,
/* FP:symbol.rs-1371 */         maxnumf64,
/* FP:symbol.rs-1372 */         maxnumf128,
/* FP:symbol.rs-1373 */         may_dangle,
/* FP:symbol.rs-1374 */         may_unwind,
/* FP:symbol.rs-1375 */         maybe_uninit,
/* FP:symbol.rs-1376 */         maybe_uninit_uninit,
/* FP:symbol.rs-1377 */         maybe_uninit_zeroed,
/* FP:symbol.rs-1378 */         mem_align_of,
/* FP:symbol.rs-1379 */         mem_discriminant,
/* FP:symbol.rs-1380 */         mem_drop,
/* FP:symbol.rs-1381 */         mem_forget,
/* FP:symbol.rs-1382 */         mem_replace,
/* FP:symbol.rs-1383 */         mem_size_of,
/* FP:symbol.rs-1384 */         mem_size_of_val,
/* FP:symbol.rs-1385 */         mem_swap,
/* FP:symbol.rs-1386 */         mem_uninitialized,
/* FP:symbol.rs-1387 */         mem_variant_count,
/* FP:symbol.rs-1388 */         mem_zeroed,
/* FP:symbol.rs-1389 */         member_constraints,
/* FP:symbol.rs-1390 */         memory,
/* FP:symbol.rs-1391 */         memtag,
/* FP:symbol.rs-1392 */         message,
/* FP:symbol.rs-1393 */         meta,
/* FP:symbol.rs-1394 */         meta_sized,
/* FP:symbol.rs-1395 */         metadata_type,
/* FP:symbol.rs-1396 */         min_const_fn,
/* FP:symbol.rs-1397 */         min_const_generics,
/* FP:symbol.rs-1398 */         min_const_unsafe_fn,
/* FP:symbol.rs-1399 */         min_exhaustive_patterns,
/* FP:symbol.rs-1400 */         min_generic_const_args,
/* FP:symbol.rs-1401 */         min_specialization,
/* FP:symbol.rs-1402 */         min_type_alias_impl_trait,
/* FP:symbol.rs-1403 */         minimumf16,
/* FP:symbol.rs-1404 */         minimumf32,
/* FP:symbol.rs-1405 */         minimumf64,
/* FP:symbol.rs-1406 */         minimumf128,
/* FP:symbol.rs-1407 */         minnumf16,
/* FP:symbol.rs-1408 */         minnumf32,
/* FP:symbol.rs-1409 */         minnumf64,
/* FP:symbol.rs-1410 */         minnumf128,
/* FP:symbol.rs-1411 */         mips_target_feature,
/* FP:symbol.rs-1412 */         mir_assume,
/* FP:symbol.rs-1413 */         mir_basic_block,
/* FP:symbol.rs-1414 */         mir_call,
/* FP:symbol.rs-1415 */         mir_cast_ptr_to_ptr,
/* FP:symbol.rs-1416 */         mir_cast_transmute,
/* FP:symbol.rs-1417 */         mir_checked,
/* FP:symbol.rs-1418 */         mir_copy_for_deref,
/* FP:symbol.rs-1419 */         mir_debuginfo,
/* FP:symbol.rs-1420 */         mir_deinit,
/* FP:symbol.rs-1421 */         mir_discriminant,
/* FP:symbol.rs-1422 */         mir_drop,
/* FP:symbol.rs-1423 */         mir_field,
/* FP:symbol.rs-1424 */         mir_goto,
/* FP:symbol.rs-1425 */         mir_len,
/* FP:symbol.rs-1426 */         mir_make_place,
/* FP:symbol.rs-1427 */         mir_move,
/* FP:symbol.rs-1428 */         mir_offset,
/* FP:symbol.rs-1429 */         mir_ptr_metadata,
/* FP:symbol.rs-1430 */         mir_retag,
/* FP:symbol.rs-1431 */         mir_return,
/* FP:symbol.rs-1432 */         mir_return_to,
/* FP:symbol.rs-1433 */         mir_set_discriminant,
/* FP:symbol.rs-1434 */         mir_static,
/* FP:symbol.rs-1435 */         mir_static_mut,
/* FP:symbol.rs-1436 */         mir_storage_dead,
/* FP:symbol.rs-1437 */         mir_storage_live,
/* FP:symbol.rs-1438 */         mir_tail_call,
/* FP:symbol.rs-1439 */         mir_unreachable,
/* FP:symbol.rs-1440 */         mir_unwind_cleanup,
/* FP:symbol.rs-1441 */         mir_unwind_continue,
/* FP:symbol.rs-1442 */         mir_unwind_resume,
/* FP:symbol.rs-1443 */         mir_unwind_terminate,
/* FP:symbol.rs-1444 */         mir_unwind_terminate_reason,
/* FP:symbol.rs-1445 */         mir_unwind_unreachable,
/* FP:symbol.rs-1446 */         mir_variant,
/* FP:symbol.rs-1447 */         miri,
/* FP:symbol.rs-1448 */         mmx_reg,
/* FP:symbol.rs-1449 */         modifiers,
/* FP:symbol.rs-1450 */         module,
/* FP:symbol.rs-1451 */         module_path,
/* FP:symbol.rs-1452 */         more_maybe_bounds,
/* FP:symbol.rs-1453 */         more_qualified_paths,
/* FP:symbol.rs-1454 */         more_struct_aliases,
/* FP:symbol.rs-1455 */         movbe_target_feature,
/* FP:symbol.rs-1456 */         move_ref_pattern,
/* FP:symbol.rs-1457 */         move_size_limit,
/* FP:symbol.rs-1458 */         movrs_target_feature,
/* FP:symbol.rs-1459 */         mul,
/* FP:symbol.rs-1460 */         mul_assign,
/* FP:symbol.rs-1461 */         mul_with_overflow,
/* FP:symbol.rs-1462 */         multiple_supertrait_upcastable,
/* FP:symbol.rs-1463 */         must_not_suspend,
/* FP:symbol.rs-1464 */         must_use,
/* FP:symbol.rs-1465 */         mut_preserve_binding_mode_2024,
/* FP:symbol.rs-1466 */         mut_ref,
/* FP:symbol.rs-1467 */         naked,
/* FP:symbol.rs-1468 */         naked_asm,
/* FP:symbol.rs-1469 */         naked_functions,
/* FP:symbol.rs-1470 */         naked_functions_rustic_abi,
/* FP:symbol.rs-1471 */         naked_functions_target_feature,
/* FP:symbol.rs-1472 */         name,
/* FP:symbol.rs-1473 */         names,
/* FP:symbol.rs-1474 */         native_link_modifiers,
/* FP:symbol.rs-1475 */         native_link_modifiers_as_needed,
/* FP:symbol.rs-1476 */         native_link_modifiers_bundle,
/* FP:symbol.rs-1477 */         native_link_modifiers_verbatim,
/* FP:symbol.rs-1478 */         native_link_modifiers_whole_archive,
/* FP:symbol.rs-1479 */         natvis_file,
/* FP:symbol.rs-1480 */         ne,
/* FP:symbol.rs-1481 */         needs_allocator,
/* FP:symbol.rs-1482 */         needs_drop,
/* FP:symbol.rs-1483 */         needs_panic_runtime,
/* FP:symbol.rs-1484 */         neg,
/* FP:symbol.rs-1485 */         negate_unsigned,
/* FP:symbol.rs-1486 */         negative_bounds,
/* FP:symbol.rs-1487 */         negative_impls,
/* FP:symbol.rs-1488 */         neon,
/* FP:symbol.rs-1489 */         nested,
/* FP:symbol.rs-1490 */         never,
/* FP:symbol.rs-1491 */         never_patterns,
/* FP:symbol.rs-1492 */         never_type,
/* FP:symbol.rs-1493 */         never_type_fallback,
/* FP:symbol.rs-1494 */         new,
/* FP:symbol.rs-1495 */         new_binary,
/* FP:symbol.rs-1496 */         new_const,
/* FP:symbol.rs-1497 */         new_debug,
/* FP:symbol.rs-1498 */         new_debug_noop,
/* FP:symbol.rs-1499 */         new_display,
/* FP:symbol.rs-1500 */         new_lower_exp,
/* FP:symbol.rs-1501 */         new_lower_hex,
/* FP:symbol.rs-1502 */         new_octal,
/* FP:symbol.rs-1503 */         new_pointer,
/* FP:symbol.rs-1504 */         new_range,
/* FP:symbol.rs-1505 */         new_unchecked,
/* FP:symbol.rs-1506 */         new_upper_exp,
/* FP:symbol.rs-1507 */         new_upper_hex,
/* FP:symbol.rs-1508 */         new_v1,
/* FP:symbol.rs-1509 */         new_v1_formatted,
/* FP:symbol.rs-1510 */         next,
/* FP:symbol.rs-1511 */         niko,
/* FP:symbol.rs-1512 */         nll,
/* FP:symbol.rs-1513 */         no,
/* FP:symbol.rs-1514 */         no_builtins,
/* FP:symbol.rs-1515 */         no_core,
/* FP:symbol.rs-1516 */         no_coverage,
/* FP:symbol.rs-1517 */         no_crate_inject,
/* FP:symbol.rs-1518 */         no_debug,
/* FP:symbol.rs-1519 */         no_default_passes,
/* FP:symbol.rs-1520 */         no_implicit_prelude,
/* FP:symbol.rs-1521 */         no_inline,
/* FP:symbol.rs-1522 */         no_link,
/* FP:symbol.rs-1523 */         no_main,
/* FP:symbol.rs-1524 */         no_mangle,
/* FP:symbol.rs-1525 */         no_sanitize,
/* FP:symbol.rs-1526 */         no_stack_check,
/* FP:symbol.rs-1527 */         no_std,
/* FP:symbol.rs-1528 */         nomem,
/* FP:symbol.rs-1529 */         non_ascii_idents,
/* FP:symbol.rs-1530 */         non_exhaustive,
/* FP:symbol.rs-1531 */         non_exhaustive_omitted_patterns_lint,
/* FP:symbol.rs-1532 */         non_lifetime_binders,
/* FP:symbol.rs-1533 */         non_modrs_mods,
/* FP:symbol.rs-1534 */         none,
/* FP:symbol.rs-1535 */         nontemporal_store,
/* FP:symbol.rs-1536 */         noop_method_borrow,
/* FP:symbol.rs-1537 */         noop_method_clone,
/* FP:symbol.rs-1538 */         noop_method_deref,
/* FP:symbol.rs-1539 */         noprefix,
/* FP:symbol.rs-1540 */         noreturn,
/* FP:symbol.rs-1541 */         nostack,
/* FP:symbol.rs-1542 */         not,
/* FP:symbol.rs-1543 */         notable_trait,
/* FP:symbol.rs-1544 */         note,
/* FP:symbol.rs-1545 */         nvptx_target_feature,
/* FP:symbol.rs-1546 */         object_safe_for_dispatch,
/* FP:symbol.rs-1547 */         of,
/* FP:symbol.rs-1548 */         off,
/* FP:symbol.rs-1549 */         offset,
/* FP:symbol.rs-1550 */         offset_of,
/* FP:symbol.rs-1551 */         offset_of_enum,
/* FP:symbol.rs-1552 */         offset_of_nested,
/* FP:symbol.rs-1553 */         offset_of_slice,
/* FP:symbol.rs-1554 */         ok_or_else,
/* FP:symbol.rs-1555 */         old_name,
/* FP:symbol.rs-1556 */         omit_gdb_pretty_printer_section,
/* FP:symbol.rs-1557 */         on,
/* FP:symbol.rs-1558 */         on_unimplemented,
/* FP:symbol.rs-1559 */         opaque,
/* FP:symbol.rs-1560 */         opaque_module_name_placeholder: "<opaque>",
/* FP:symbol.rs-1561 */         open_options_new,
/* FP:symbol.rs-1562 */         ops,
/* FP:symbol.rs-1563 */         opt_out_copy,
/* FP:symbol.rs-1564 */         optimize,
/* FP:symbol.rs-1565 */         optimize_attribute,
/* FP:symbol.rs-1566 */         optimized,
/* FP:symbol.rs-1567 */         optin_builtin_traits,
/* FP:symbol.rs-1568 */         option,
/* FP:symbol.rs-1569 */         option_env,
/* FP:symbol.rs-1570 */         option_expect,
/* FP:symbol.rs-1571 */         option_unwrap,
/* FP:symbol.rs-1572 */         options,
/* FP:symbol.rs-1573 */         or,
/* FP:symbol.rs-1574 */         or_patterns,
/* FP:symbol.rs-1575 */         ord_cmp_method,
/* FP:symbol.rs-1576 */         os_str_to_os_string,
/* FP:symbol.rs-1577 */         os_string_as_os_str,
/* FP:symbol.rs-1578 */         other,
/* FP:symbol.rs-1579 */         out,
/* FP:symbol.rs-1580 */         overflow_checks,
/* FP:symbol.rs-1581 */         overlapping_marker_traits,
/* FP:symbol.rs-1582 */         owned_box,
/* FP:symbol.rs-1583 */         packed,
/* FP:symbol.rs-1584 */         packed_bundled_libs,
/* FP:symbol.rs-1585 */         panic,
/* FP:symbol.rs-1586 */         panic_2015,
/* FP:symbol.rs-1587 */         panic_2021,
/* FP:symbol.rs-1588 */         panic_abort,
/* FP:symbol.rs-1589 */         panic_any,
/* FP:symbol.rs-1590 */         panic_bounds_check,
/* FP:symbol.rs-1591 */         panic_cannot_unwind,
/* FP:symbol.rs-1592 */         panic_const_add_overflow,
/* FP:symbol.rs-1593 */         panic_const_async_fn_resumed,
/* FP:symbol.rs-1594 */         panic_const_async_fn_resumed_drop,
/* FP:symbol.rs-1595 */         panic_const_async_fn_resumed_panic,
/* FP:symbol.rs-1596 */         panic_const_async_gen_fn_resumed,
/* FP:symbol.rs-1597 */         panic_const_async_gen_fn_resumed_drop,
/* FP:symbol.rs-1598 */         panic_const_async_gen_fn_resumed_panic,
/* FP:symbol.rs-1599 */         panic_const_coroutine_resumed,
/* FP:symbol.rs-1600 */         panic_const_coroutine_resumed_drop,
/* FP:symbol.rs-1601 */         panic_const_coroutine_resumed_panic,
/* FP:symbol.rs-1602 */         panic_const_div_by_zero,
/* FP:symbol.rs-1603 */         panic_const_div_overflow,
/* FP:symbol.rs-1604 */         panic_const_gen_fn_none,
/* FP:symbol.rs-1605 */         panic_const_gen_fn_none_drop,
/* FP:symbol.rs-1606 */         panic_const_gen_fn_none_panic,
/* FP:symbol.rs-1607 */         panic_const_mul_overflow,
/* FP:symbol.rs-1608 */         panic_const_neg_overflow,
/* FP:symbol.rs-1609 */         panic_const_rem_by_zero,
/* FP:symbol.rs-1610 */         panic_const_rem_overflow,
/* FP:symbol.rs-1611 */         panic_const_shl_overflow,
/* FP:symbol.rs-1612 */         panic_const_shr_overflow,
/* FP:symbol.rs-1613 */         panic_const_sub_overflow,
/* FP:symbol.rs-1614 */         panic_display,
/* FP:symbol.rs-1615 */         panic_fmt,
/* FP:symbol.rs-1616 */         panic_handler,
/* FP:symbol.rs-1617 */         panic_impl,
/* FP:symbol.rs-1618 */         panic_implementation,
/* FP:symbol.rs-1619 */         panic_in_cleanup,
/* FP:symbol.rs-1620 */         panic_info,
/* FP:symbol.rs-1621 */         panic_invalid_enum_construction,
/* FP:symbol.rs-1622 */         panic_location,
/* FP:symbol.rs-1623 */         panic_misaligned_pointer_dereference,
/* FP:symbol.rs-1624 */         panic_nounwind,
/* FP:symbol.rs-1625 */         panic_null_pointer_dereference,
/* FP:symbol.rs-1626 */         panic_runtime,
/* FP:symbol.rs-1627 */         panic_str_2015,
/* FP:symbol.rs-1628 */         panic_unwind,
/* FP:symbol.rs-1629 */         panicking,
/* FP:symbol.rs-1630 */         param_attrs,
/* FP:symbol.rs-1631 */         parent_label,
/* FP:symbol.rs-1632 */         partial_cmp,
/* FP:symbol.rs-1633 */         partial_ord,
/* FP:symbol.rs-1634 */         passes,
/* FP:symbol.rs-1635 */         pat,
/* FP:symbol.rs-1636 */         pat_param,
/* FP:symbol.rs-1637 */         patchable_function_entry,
/* FP:symbol.rs-1638 */         path,
/* FP:symbol.rs-1639 */         path_main_separator,
/* FP:symbol.rs-1640 */         path_to_pathbuf,
/* FP:symbol.rs-1641 */         pathbuf_as_path,
/* FP:symbol.rs-1642 */         pattern_complexity_limit,
/* FP:symbol.rs-1643 */         pattern_parentheses,
/* FP:symbol.rs-1644 */         pattern_type,
/* FP:symbol.rs-1645 */         pattern_type_range_trait,
/* FP:symbol.rs-1646 */         pattern_types,
/* FP:symbol.rs-1647 */         permissions_from_mode,
/* FP:symbol.rs-1648 */         phantom_data,
/* FP:symbol.rs-1649 */         phase,
/* FP:symbol.rs-1650 */         pic,
/* FP:symbol.rs-1651 */         pie,
/* FP:symbol.rs-1652 */         pin,
/* FP:symbol.rs-1653 */         pin_ergonomics,
/* FP:symbol.rs-1654 */         pin_macro,
/* FP:symbol.rs-1655 */         platform_intrinsics,
/* FP:symbol.rs-1656 */         plugin,
/* FP:symbol.rs-1657 */         plugin_registrar,
/* FP:symbol.rs-1658 */         plugins,
/* FP:symbol.rs-1659 */         pointee,
/* FP:symbol.rs-1660 */         pointee_sized,
/* FP:symbol.rs-1661 */         pointee_trait,
/* FP:symbol.rs-1662 */         pointer,
/* FP:symbol.rs-1663 */         poll,
/* FP:symbol.rs-1664 */         poll_next,
/* FP:symbol.rs-1665 */         position,
/* FP:symbol.rs-1666 */         post_cleanup: "post-cleanup",
/* FP:symbol.rs-1667 */         post_dash_lto: "post-lto",
/* FP:symbol.rs-1668 */         postfix_match,
/* FP:symbol.rs-1669 */         powerpc_target_feature,
/* FP:symbol.rs-1670 */         powf16,
/* FP:symbol.rs-1671 */         powf32,
/* FP:symbol.rs-1672 */         powf64,
/* FP:symbol.rs-1673 */         powf128,
/* FP:symbol.rs-1674 */         powif16,
/* FP:symbol.rs-1675 */         powif32,
/* FP:symbol.rs-1676 */         powif64,
/* FP:symbol.rs-1677 */         powif128,
/* FP:symbol.rs-1678 */         pre_dash_lto: "pre-lto",
/* FP:symbol.rs-1679 */         precise_capturing,
/* FP:symbol.rs-1680 */         precise_capturing_in_traits,
/* FP:symbol.rs-1681 */         precise_pointer_size_matching,
/* FP:symbol.rs-1682 */         precision,
/* FP:symbol.rs-1683 */         pref_align_of,
/* FP:symbol.rs-1684 */         prefetch_read_data,
/* FP:symbol.rs-1685 */         prefetch_read_instruction,
/* FP:symbol.rs-1686 */         prefetch_write_data,
/* FP:symbol.rs-1687 */         prefetch_write_instruction,
/* FP:symbol.rs-1688 */         prefix_nops,
/* FP:symbol.rs-1689 */         preg,
/* FP:symbol.rs-1690 */         prelude,
/* FP:symbol.rs-1691 */         prelude_import,
/* FP:symbol.rs-1692 */         preserves_flags,
/* FP:symbol.rs-1693 */         prfchw_target_feature,
/* FP:symbol.rs-1694 */         print_macro,
/* FP:symbol.rs-1695 */         println_macro,
/* FP:symbol.rs-1696 */         proc_dash_macro: "proc-macro",
/* FP:symbol.rs-1697 */         proc_macro,
/* FP:symbol.rs-1698 */         proc_macro_attribute,
/* FP:symbol.rs-1699 */         proc_macro_derive,
/* FP:symbol.rs-1700 */         proc_macro_expr,
/* FP:symbol.rs-1701 */         proc_macro_gen,
/* FP:symbol.rs-1702 */         proc_macro_hygiene,
/* FP:symbol.rs-1703 */         proc_macro_internals,
/* FP:symbol.rs-1704 */         proc_macro_mod,
/* FP:symbol.rs-1705 */         proc_macro_non_items,
/* FP:symbol.rs-1706 */         proc_macro_path_invoc,
/* FP:symbol.rs-1707 */         process_abort,
/* FP:symbol.rs-1708 */         process_exit,
/* FP:symbol.rs-1709 */         profiler_builtins,
/* FP:symbol.rs-1710 */         profiler_runtime,
/* FP:symbol.rs-1711 */         ptr,
/* FP:symbol.rs-1712 */         ptr_cast,
/* FP:symbol.rs-1713 */         ptr_cast_const,
/* FP:symbol.rs-1714 */         ptr_cast_mut,
/* FP:symbol.rs-1715 */         ptr_const_is_null,
/* FP:symbol.rs-1716 */         ptr_copy,
/* FP:symbol.rs-1717 */         ptr_copy_nonoverlapping,
/* FP:symbol.rs-1718 */         ptr_eq,
/* FP:symbol.rs-1719 */         ptr_from_ref,
/* FP:symbol.rs-1720 */         ptr_guaranteed_cmp,
/* FP:symbol.rs-1721 */         ptr_is_null,
/* FP:symbol.rs-1722 */         ptr_mask,
/* FP:symbol.rs-1723 */         ptr_metadata,
/* FP:symbol.rs-1724 */         ptr_null,
/* FP:symbol.rs-1725 */         ptr_null_mut,
/* FP:symbol.rs-1726 */         ptr_offset_from,
/* FP:symbol.rs-1727 */         ptr_offset_from_unsigned,
/* FP:symbol.rs-1728 */         ptr_read,
/* FP:symbol.rs-1729 */         ptr_read_unaligned,
/* FP:symbol.rs-1730 */         ptr_read_volatile,
/* FP:symbol.rs-1731 */         ptr_replace,
/* FP:symbol.rs-1732 */         ptr_slice_from_raw_parts,
/* FP:symbol.rs-1733 */         ptr_slice_from_raw_parts_mut,
/* FP:symbol.rs-1734 */         ptr_swap,
/* FP:symbol.rs-1735 */         ptr_swap_nonoverlapping,
/* FP:symbol.rs-1736 */         ptr_write,
/* FP:symbol.rs-1737 */         ptr_write_bytes,
/* FP:symbol.rs-1738 */         ptr_write_unaligned,
/* FP:symbol.rs-1739 */         ptr_write_volatile,
/* FP:symbol.rs-1740 */         pub_macro_rules,
/* FP:symbol.rs-1741 */         pub_restricted,
/* FP:symbol.rs-1742 */         public,
/* FP:symbol.rs-1743 */         pure,
/* FP:symbol.rs-1744 */         pushpop_unsafe,
/* FP:symbol.rs-1745 */         qreg,
/* FP:symbol.rs-1746 */         qreg_low4,
/* FP:symbol.rs-1747 */         qreg_low8,
/* FP:symbol.rs-1748 */         quad_precision_float,
/* FP:symbol.rs-1749 */         question_mark,
/* FP:symbol.rs-1750 */         quote,
/* FP:symbol.rs-1751 */         range_inclusive_new,
/* FP:symbol.rs-1752 */         range_step,
/* FP:symbol.rs-1753 */         raw_dash_dylib: "raw-dylib",
/* FP:symbol.rs-1754 */         raw_dylib,
/* FP:symbol.rs-1755 */         raw_dylib_elf,
/* FP:symbol.rs-1756 */         raw_eq,
/* FP:symbol.rs-1757 */         raw_identifiers,
/* FP:symbol.rs-1758 */         raw_ref_op,
/* FP:symbol.rs-1759 */         re_rebalance_coherence,
/* FP:symbol.rs-1760 */         read_enum,
/* FP:symbol.rs-1761 */         read_enum_variant,
/* FP:symbol.rs-1762 */         read_enum_variant_arg,
/* FP:symbol.rs-1763 */         read_struct,
/* FP:symbol.rs-1764 */         read_struct_field,
/* FP:symbol.rs-1765 */         read_via_copy,
/* FP:symbol.rs-1766 */         readonly,
/* FP:symbol.rs-1767 */         realloc,
/* FP:symbol.rs-1768 */         reason,
/* FP:symbol.rs-1769 */         reborrow,
/* FP:symbol.rs-1770 */         receiver,
/* FP:symbol.rs-1771 */         receiver_target,
/* FP:symbol.rs-1772 */         recursion_limit,
/* FP:symbol.rs-1773 */         reexport_test_harness_main,
/* FP:symbol.rs-1774 */         ref_pat_eat_one_layer_2024,
/* FP:symbol.rs-1775 */         ref_pat_eat_one_layer_2024_structural,
/* FP:symbol.rs-1776 */         ref_pat_everywhere,
/* FP:symbol.rs-1777 */         ref_unwind_safe_trait,
/* FP:symbol.rs-1778 */         reference,
/* FP:symbol.rs-1779 */         reflect,
/* FP:symbol.rs-1780 */         reg,
/* FP:symbol.rs-1781 */         reg16,
/* FP:symbol.rs-1782 */         reg32,
/* FP:symbol.rs-1783 */         reg64,
/* FP:symbol.rs-1784 */         reg_abcd,
/* FP:symbol.rs-1785 */         reg_addr,
/* FP:symbol.rs-1786 */         reg_byte,
/* FP:symbol.rs-1787 */         reg_data,
/* FP:symbol.rs-1788 */         reg_iw,
/* FP:symbol.rs-1789 */         reg_nonzero,
/* FP:symbol.rs-1790 */         reg_pair,
/* FP:symbol.rs-1791 */         reg_ptr,
/* FP:symbol.rs-1792 */         reg_upper,
/* FP:symbol.rs-1793 */         register_attr,
/* FP:symbol.rs-1794 */         register_tool,
/* FP:symbol.rs-1795 */         relaxed_adts,
/* FP:symbol.rs-1796 */         relaxed_struct_unsize,
/* FP:symbol.rs-1797 */         relocation_model,
/* FP:symbol.rs-1798 */         rem,
/* FP:symbol.rs-1799 */         rem_assign,
/* FP:symbol.rs-1800 */         repr,
/* FP:symbol.rs-1801 */         repr128,
/* FP:symbol.rs-1802 */         repr_align,
/* FP:symbol.rs-1803 */         repr_align_enum,
/* FP:symbol.rs-1804 */         repr_packed,
/* FP:symbol.rs-1805 */         repr_simd,
/* FP:symbol.rs-1806 */         repr_transparent,
/* FP:symbol.rs-1807 */         require,
/* FP:symbol.rs-1808 */         reserve_x18: "reserve-x18",
/* FP:symbol.rs-1809 */         residual,
/* FP:symbol.rs-1810 */         result,
/* FP:symbol.rs-1811 */         result_ffi_guarantees,
/* FP:symbol.rs-1812 */         result_ok_method,
/* FP:symbol.rs-1813 */         resume,
/* FP:symbol.rs-1814 */         return_position_impl_trait_in_trait,
/* FP:symbol.rs-1815 */         return_type_notation,
/* FP:symbol.rs-1816 */         riscv_target_feature,
/* FP:symbol.rs-1817 */         rlib,
/* FP:symbol.rs-1818 */         ropi,
/* FP:symbol.rs-1819 */         ropi_rwpi: "ropi-rwpi",
/* FP:symbol.rs-1820 */         rotate_left,
/* FP:symbol.rs-1821 */         rotate_right,
/* FP:symbol.rs-1822 */         round_ties_even_f16,
/* FP:symbol.rs-1823 */         round_ties_even_f32,
/* FP:symbol.rs-1824 */         round_ties_even_f64,
/* FP:symbol.rs-1825 */         round_ties_even_f128,
/* FP:symbol.rs-1826 */         roundf16,
/* FP:symbol.rs-1827 */         roundf32,
/* FP:symbol.rs-1828 */         roundf64,
/* FP:symbol.rs-1829 */         roundf128,
/* FP:symbol.rs-1830 */         rt,
/* FP:symbol.rs-1831 */         rtm_target_feature,
/* FP:symbol.rs-1832 */         runtime,
/* FP:symbol.rs-1833 */         rust,
/* FP:symbol.rs-1834 */         rust_2015,
/* FP:symbol.rs-1835 */         rust_2018,
/* FP:symbol.rs-1836 */         rust_2018_preview,
/* FP:symbol.rs-1837 */         rust_2021,
/* FP:symbol.rs-1838 */         rust_2024,
/* FP:symbol.rs-1839 */         rust_analyzer,
/* FP:symbol.rs-1840 */         rust_begin_unwind,
/* FP:symbol.rs-1841 */         rust_cold_cc,
/* FP:symbol.rs-1842 */         rust_eh_catch_typeinfo,
/* FP:symbol.rs-1843 */         rust_eh_personality,
/* FP:symbol.rs-1844 */         rust_future,
/* FP:symbol.rs-1845 */         rust_logo,
/* FP:symbol.rs-1846 */         rust_out,
/* FP:symbol.rs-1847 */         rustc,
/* FP:symbol.rs-1848 */         rustc_abi,
/* FP:symbol.rs-1849 */         // FIXME(#82232, #143834): temporary name to mitigate `#[align]` nameres ambiguity
/* FP:symbol.rs-1850 */         rustc_align,
/* FP:symbol.rs-1851 */         rustc_align_static,
/* FP:symbol.rs-1852 */         rustc_allocator,
/* FP:symbol.rs-1853 */         rustc_allocator_zeroed,
/* FP:symbol.rs-1854 */         rustc_allow_const_fn_unstable,
/* FP:symbol.rs-1855 */         rustc_allow_incoherent_impl,
/* FP:symbol.rs-1856 */         rustc_allowed_through_unstable_modules,
/* FP:symbol.rs-1857 */         rustc_as_ptr,
/* FP:symbol.rs-1858 */         rustc_attrs,
/* FP:symbol.rs-1859 */         rustc_autodiff,
/* FP:symbol.rs-1860 */         rustc_builtin_macro,
/* FP:symbol.rs-1861 */         rustc_capture_analysis,
/* FP:symbol.rs-1862 */         rustc_clean,
/* FP:symbol.rs-1863 */         rustc_coherence_is_core,
/* FP:symbol.rs-1864 */         rustc_coinductive,
/* FP:symbol.rs-1865 */         rustc_confusables,
/* FP:symbol.rs-1866 */         rustc_const_stable,
/* FP:symbol.rs-1867 */         rustc_const_stable_indirect,
/* FP:symbol.rs-1868 */         rustc_const_unstable,
/* FP:symbol.rs-1869 */         rustc_conversion_suggestion,
/* FP:symbol.rs-1870 */         rustc_deallocator,
/* FP:symbol.rs-1871 */         rustc_def_path,
/* FP:symbol.rs-1872 */         rustc_default_body_unstable,
/* FP:symbol.rs-1873 */         rustc_delayed_bug_from_inside_query,
/* FP:symbol.rs-1874 */         rustc_deny_explicit_impl,
/* FP:symbol.rs-1875 */         rustc_deprecated_safe_2024,
/* FP:symbol.rs-1876 */         rustc_diagnostic_item,
/* FP:symbol.rs-1877 */         rustc_diagnostic_macros,
/* FP:symbol.rs-1878 */         rustc_dirty,
/* FP:symbol.rs-1879 */         rustc_do_not_const_check,
/* FP:symbol.rs-1880 */         rustc_do_not_implement_via_object,
/* FP:symbol.rs-1881 */         rustc_doc_primitive,
/* FP:symbol.rs-1882 */         rustc_driver,
/* FP:symbol.rs-1883 */         rustc_dummy,
/* FP:symbol.rs-1884 */         rustc_dump_def_parents,
/* FP:symbol.rs-1885 */         rustc_dump_item_bounds,
/* FP:symbol.rs-1886 */         rustc_dump_predicates,
/* FP:symbol.rs-1887 */         rustc_dump_user_args,
/* FP:symbol.rs-1888 */         rustc_dump_vtable,
/* FP:symbol.rs-1889 */         rustc_effective_visibility,
/* FP:symbol.rs-1890 */         rustc_evaluate_where_clauses,
/* FP:symbol.rs-1891 */         rustc_expected_cgu_reuse,
/* FP:symbol.rs-1892 */         rustc_force_inline,
/* FP:symbol.rs-1893 */         rustc_has_incoherent_inherent_impls,
/* FP:symbol.rs-1894 */         rustc_hidden_type_of_opaques,
/* FP:symbol.rs-1895 */         rustc_if_this_changed,
/* FP:symbol.rs-1896 */         rustc_inherit_overflow_checks,
/* FP:symbol.rs-1897 */         rustc_insignificant_dtor,
/* FP:symbol.rs-1898 */         rustc_intrinsic,
/* FP:symbol.rs-1899 */         rustc_intrinsic_const_stable_indirect,
/* FP:symbol.rs-1900 */         rustc_layout,
/* FP:symbol.rs-1901 */         rustc_layout_scalar_valid_range_end,
/* FP:symbol.rs-1902 */         rustc_layout_scalar_valid_range_start,
/* FP:symbol.rs-1903 */         rustc_legacy_const_generics,
/* FP:symbol.rs-1904 */         rustc_lint_diagnostics,
/* FP:symbol.rs-1905 */         rustc_lint_opt_deny_field_access,
/* FP:symbol.rs-1906 */         rustc_lint_opt_ty,
/* FP:symbol.rs-1907 */         rustc_lint_query_instability,
/* FP:symbol.rs-1908 */         rustc_lint_untracked_query_information,
/* FP:symbol.rs-1909 */         rustc_macro_transparency,
/* FP:symbol.rs-1910 */         rustc_main,
/* FP:symbol.rs-1911 */         rustc_mir,
/* FP:symbol.rs-1912 */         rustc_must_implement_one_of,
/* FP:symbol.rs-1913 */         rustc_never_returns_null_ptr,
/* FP:symbol.rs-1914 */         rustc_never_type_options,
/* FP:symbol.rs-1915 */         rustc_no_implicit_autorefs,
/* FP:symbol.rs-1916 */         rustc_no_implicit_bounds,
/* FP:symbol.rs-1917 */         rustc_no_mir_inline,
/* FP:symbol.rs-1918 */         rustc_nonnull_optimization_guaranteed,
/* FP:symbol.rs-1919 */         rustc_nounwind,
/* FP:symbol.rs-1920 */         rustc_object_lifetime_default,
/* FP:symbol.rs-1921 */         rustc_on_unimplemented,
/* FP:symbol.rs-1922 */         rustc_outlives,
/* FP:symbol.rs-1923 */         rustc_paren_sugar,
/* FP:symbol.rs-1924 */         rustc_partition_codegened,
/* FP:symbol.rs-1925 */         rustc_partition_reused,
/* FP:symbol.rs-1926 */         rustc_pass_by_value,
/* FP:symbol.rs-1927 */         rustc_peek,
/* FP:symbol.rs-1928 */         rustc_peek_liveness,
/* FP:symbol.rs-1929 */         rustc_peek_maybe_init,
/* FP:symbol.rs-1930 */         rustc_peek_maybe_uninit,
/* FP:symbol.rs-1931 */         rustc_preserve_ub_checks,
/* FP:symbol.rs-1932 */         rustc_private,
/* FP:symbol.rs-1933 */         rustc_proc_macro_decls,
/* FP:symbol.rs-1934 */         rustc_promotable,
/* FP:symbol.rs-1935 */         rustc_pub_transparent,
/* FP:symbol.rs-1936 */         rustc_reallocator,
/* FP:symbol.rs-1937 */         rustc_regions,
/* FP:symbol.rs-1938 */         rustc_reservation_impl,
/* FP:symbol.rs-1939 */         rustc_serialize,
/* FP:symbol.rs-1940 */         rustc_skip_during_method_dispatch,
/* FP:symbol.rs-1941 */         rustc_specialization_trait,
/* FP:symbol.rs-1942 */         rustc_std_internal_symbol,
/* FP:symbol.rs-1943 */         rustc_strict_coherence,
/* FP:symbol.rs-1944 */         rustc_symbol_name,
/* FP:symbol.rs-1945 */         rustc_test_marker,
/* FP:symbol.rs-1946 */         rustc_then_this_would_need,
/* FP:symbol.rs-1947 */         rustc_trivial_field_reads,
/* FP:symbol.rs-1948 */         rustc_unsafe_specialization_marker,
/* FP:symbol.rs-1949 */         rustc_variance,
/* FP:symbol.rs-1950 */         rustc_variance_of_opaques,
/* FP:symbol.rs-1951 */         rustdoc,
/* FP:symbol.rs-1952 */         rustdoc_internals,
/* FP:symbol.rs-1953 */         rustdoc_missing_doc_code_examples,
/* FP:symbol.rs-1954 */         rustfmt,
/* FP:symbol.rs-1955 */         rvalue_static_promotion,
/* FP:symbol.rs-1956 */         rwpi,
/* FP:symbol.rs-1957 */         s,
/* FP:symbol.rs-1958 */         s390x_target_feature,
/* FP:symbol.rs-1959 */         safety,
/* FP:symbol.rs-1960 */         sanitize,
/* FP:symbol.rs-1961 */         sanitizer_cfi_generalize_pointers,
/* FP:symbol.rs-1962 */         sanitizer_cfi_normalize_integers,
/* FP:symbol.rs-1963 */         sanitizer_runtime,
/* FP:symbol.rs-1964 */         saturating_add,
/* FP:symbol.rs-1965 */         saturating_div,
/* FP:symbol.rs-1966 */         saturating_sub,
/* FP:symbol.rs-1967 */         sdylib,
/* FP:symbol.rs-1968 */         search_unbox,
/* FP:symbol.rs-1969 */         select_unpredictable,
/* FP:symbol.rs-1970 */         self_in_typedefs,
/* FP:symbol.rs-1971 */         self_struct_ctor,
/* FP:symbol.rs-1972 */         semiopaque,
/* FP:symbol.rs-1973 */         semitransparent,
/* FP:symbol.rs-1974 */         sha2,
/* FP:symbol.rs-1975 */         sha3,
/* FP:symbol.rs-1976 */         sha512_sm_x86,
/* FP:symbol.rs-1977 */         shadow_call_stack,
/* FP:symbol.rs-1978 */         shallow,
/* FP:symbol.rs-1979 */         shl,
/* FP:symbol.rs-1980 */         shl_assign,
/* FP:symbol.rs-1981 */         shorter_tail_lifetimes,
/* FP:symbol.rs-1982 */         should_panic,
/* FP:symbol.rs-1983 */         shr,
/* FP:symbol.rs-1984 */         shr_assign,
/* FP:symbol.rs-1985 */         sig_dfl,
/* FP:symbol.rs-1986 */         sig_ign,
/* FP:symbol.rs-1987 */         simd,
/* FP:symbol.rs-1988 */         simd_add,
/* FP:symbol.rs-1989 */         simd_and,
/* FP:symbol.rs-1990 */         simd_arith_offset,
/* FP:symbol.rs-1991 */         simd_as,
/* FP:symbol.rs-1992 */         simd_bitmask,
/* FP:symbol.rs-1993 */         simd_bitreverse,
/* FP:symbol.rs-1994 */         simd_bswap,
/* FP:symbol.rs-1995 */         simd_cast,
/* FP:symbol.rs-1996 */         simd_cast_ptr,
/* FP:symbol.rs-1997 */         simd_ceil,
/* FP:symbol.rs-1998 */         simd_ctlz,
/* FP:symbol.rs-1999 */         simd_ctpop,
/* FP:symbol.rs-2000 */         simd_cttz,
/* FP:symbol.rs-2001 */         simd_div,
/* FP:symbol.rs-2002 */         simd_eq,
/* FP:symbol.rs-2003 */         simd_expose_provenance,
/* FP:symbol.rs-2004 */         simd_extract,
/* FP:symbol.rs-2005 */         simd_extract_dyn,
/* FP:symbol.rs-2006 */         simd_fabs,
/* FP:symbol.rs-2007 */         simd_fcos,
/* FP:symbol.rs-2008 */         simd_fexp,
/* FP:symbol.rs-2009 */         simd_fexp2,
/* FP:symbol.rs-2010 */         simd_ffi,
/* FP:symbol.rs-2011 */         simd_flog,
/* FP:symbol.rs-2012 */         simd_flog2,
/* FP:symbol.rs-2013 */         simd_flog10,
/* FP:symbol.rs-2014 */         simd_floor,
/* FP:symbol.rs-2015 */         simd_fma,
/* FP:symbol.rs-2016 */         simd_fmax,
/* FP:symbol.rs-2017 */         simd_fmin,
/* FP:symbol.rs-2018 */         simd_fsin,
/* FP:symbol.rs-2019 */         simd_fsqrt,
/* FP:symbol.rs-2020 */         simd_funnel_shl,
/* FP:symbol.rs-2021 */         simd_funnel_shr,
/* FP:symbol.rs-2022 */         simd_gather,
/* FP:symbol.rs-2023 */         simd_ge,
/* FP:symbol.rs-2024 */         simd_gt,
/* FP:symbol.rs-2025 */         simd_insert,
/* FP:symbol.rs-2026 */         simd_insert_dyn,
/* FP:symbol.rs-2027 */         simd_le,
/* FP:symbol.rs-2028 */         simd_lt,
/* FP:symbol.rs-2029 */         simd_masked_load,
/* FP:symbol.rs-2030 */         simd_masked_store,
/* FP:symbol.rs-2031 */         simd_mul,
/* FP:symbol.rs-2032 */         simd_ne,
/* FP:symbol.rs-2033 */         simd_neg,
/* FP:symbol.rs-2034 */         simd_or,
/* FP:symbol.rs-2035 */         simd_reduce_add_ordered,
/* FP:symbol.rs-2036 */         simd_reduce_add_unordered,
/* FP:symbol.rs-2037 */         simd_reduce_all,
/* FP:symbol.rs-2038 */         simd_reduce_and,
/* FP:symbol.rs-2039 */         simd_reduce_any,
/* FP:symbol.rs-2040 */         simd_reduce_max,
/* FP:symbol.rs-2041 */         simd_reduce_min,
/* FP:symbol.rs-2042 */         simd_reduce_mul_ordered,
/* FP:symbol.rs-2043 */         simd_reduce_mul_unordered,
/* FP:symbol.rs-2044 */         simd_reduce_or,
/* FP:symbol.rs-2045 */         simd_reduce_xor,
/* FP:symbol.rs-2046 */         simd_relaxed_fma,
/* FP:symbol.rs-2047 */         simd_rem,
/* FP:symbol.rs-2048 */         simd_round,
/* FP:symbol.rs-2049 */         simd_round_ties_even,
/* FP:symbol.rs-2050 */         simd_saturating_add,
/* FP:symbol.rs-2051 */         simd_saturating_sub,
/* FP:symbol.rs-2052 */         simd_scatter,
/* FP:symbol.rs-2053 */         simd_select,
/* FP:symbol.rs-2054 */         simd_select_bitmask,
/* FP:symbol.rs-2055 */         simd_shl,
/* FP:symbol.rs-2056 */         simd_shr,
/* FP:symbol.rs-2057 */         simd_shuffle,
/* FP:symbol.rs-2058 */         simd_shuffle_const_generic,
/* FP:symbol.rs-2059 */         simd_sub,
/* FP:symbol.rs-2060 */         simd_trunc,
/* FP:symbol.rs-2061 */         simd_with_exposed_provenance,
/* FP:symbol.rs-2062 */         simd_xor,
/* FP:symbol.rs-2063 */         since,
/* FP:symbol.rs-2064 */         sinf16,
/* FP:symbol.rs-2065 */         sinf32,
/* FP:symbol.rs-2066 */         sinf64,
/* FP:symbol.rs-2067 */         sinf128,
/* FP:symbol.rs-2068 */         size,
/* FP:symbol.rs-2069 */         size_of,
/* FP:symbol.rs-2070 */         size_of_val,
/* FP:symbol.rs-2071 */         sized,
/* FP:symbol.rs-2072 */         sized_hierarchy,
/* FP:symbol.rs-2073 */         skip,
/* FP:symbol.rs-2074 */         slice,
/* FP:symbol.rs-2075 */         slice_from_raw_parts,
/* FP:symbol.rs-2076 */         slice_from_raw_parts_mut,
/* FP:symbol.rs-2077 */         slice_from_ref,
/* FP:symbol.rs-2078 */         slice_get_unchecked,
/* FP:symbol.rs-2079 */         slice_into_vec,
/* FP:symbol.rs-2080 */         slice_iter,
/* FP:symbol.rs-2081 */         slice_len_fn,
/* FP:symbol.rs-2082 */         slice_patterns,
/* FP:symbol.rs-2083 */         slicing_syntax,
/* FP:symbol.rs-2084 */         soft,
/* FP:symbol.rs-2085 */         sparc_target_feature,
/* FP:symbol.rs-2086 */         specialization,
/* FP:symbol.rs-2087 */         speed,
/* FP:symbol.rs-2088 */         spotlight,
/* FP:symbol.rs-2089 */         sqrtf16,
/* FP:symbol.rs-2090 */         sqrtf32,
/* FP:symbol.rs-2091 */         sqrtf64,
/* FP:symbol.rs-2092 */         sqrtf128,
/* FP:symbol.rs-2093 */         sreg,
/* FP:symbol.rs-2094 */         sreg_low16,
/* FP:symbol.rs-2095 */         sse,
/* FP:symbol.rs-2096 */         sse2,
/* FP:symbol.rs-2097 */         sse4a_target_feature,
/* FP:symbol.rs-2098 */         stable,
/* FP:symbol.rs-2099 */         staged_api,
/* FP:symbol.rs-2100 */         start,
/* FP:symbol.rs-2101 */         state,
/* FP:symbol.rs-2102 */         static_align,
/* FP:symbol.rs-2103 */         static_in_const,
/* FP:symbol.rs-2104 */         static_nobundle,
/* FP:symbol.rs-2105 */         static_recursion,
/* FP:symbol.rs-2106 */         staticlib,
/* FP:symbol.rs-2107 */         std,
/* FP:symbol.rs-2108 */         std_lib_injection,
/* FP:symbol.rs-2109 */         std_panic,
/* FP:symbol.rs-2110 */         std_panic_2015_macro,
/* FP:symbol.rs-2111 */         std_panic_macro,
/* FP:symbol.rs-2112 */         stmt,
/* FP:symbol.rs-2113 */         stmt_expr_attributes,
/* FP:symbol.rs-2114 */         stop_after_dataflow,
/* FP:symbol.rs-2115 */         store,
/* FP:symbol.rs-2116 */         str,
/* FP:symbol.rs-2117 */         str_chars,
/* FP:symbol.rs-2118 */         str_ends_with,
/* FP:symbol.rs-2119 */         str_from_utf8,
/* FP:symbol.rs-2120 */         str_from_utf8_mut,
/* FP:symbol.rs-2121 */         str_from_utf8_unchecked,
/* FP:symbol.rs-2122 */         str_from_utf8_unchecked_mut,
/* FP:symbol.rs-2123 */         str_inherent_from_utf8,
/* FP:symbol.rs-2124 */         str_inherent_from_utf8_mut,
/* FP:symbol.rs-2125 */         str_inherent_from_utf8_unchecked,
/* FP:symbol.rs-2126 */         str_inherent_from_utf8_unchecked_mut,
/* FP:symbol.rs-2127 */         str_len,
/* FP:symbol.rs-2128 */         str_split_whitespace,
/* FP:symbol.rs-2129 */         str_starts_with,
/* FP:symbol.rs-2130 */         str_trim,
/* FP:symbol.rs-2131 */         str_trim_end,
/* FP:symbol.rs-2132 */         str_trim_start,
/* FP:symbol.rs-2133 */         strict_provenance_lints,
/* FP:symbol.rs-2134 */         string_as_mut_str,
/* FP:symbol.rs-2135 */         string_as_str,
/* FP:symbol.rs-2136 */         string_deref_patterns,
/* FP:symbol.rs-2137 */         string_from_utf8,
/* FP:symbol.rs-2138 */         string_insert_str,
/* FP:symbol.rs-2139 */         string_new,
/* FP:symbol.rs-2140 */         string_push_str,
/* FP:symbol.rs-2141 */         stringify,
/* FP:symbol.rs-2142 */         struct_field_attributes,
/* FP:symbol.rs-2143 */         struct_inherit,
/* FP:symbol.rs-2144 */         struct_variant,
/* FP:symbol.rs-2145 */         structural_match,
/* FP:symbol.rs-2146 */         structural_peq,
/* FP:symbol.rs-2147 */         sub,
/* FP:symbol.rs-2148 */         sub_assign,
/* FP:symbol.rs-2149 */         sub_with_overflow,
/* FP:symbol.rs-2150 */         suggestion,
/* FP:symbol.rs-2151 */         super_let,
/* FP:symbol.rs-2152 */         supertrait_item_shadowing,
/* FP:symbol.rs-2153 */         sym,
/* FP:symbol.rs-2154 */         sync,
/* FP:symbol.rs-2155 */         synthetic,
/* FP:symbol.rs-2156 */         sys_mutex_lock,
/* FP:symbol.rs-2157 */         sys_mutex_try_lock,
/* FP:symbol.rs-2158 */         sys_mutex_unlock,
/* FP:symbol.rs-2159 */         t32,
/* FP:symbol.rs-2160 */         target,
/* FP:symbol.rs-2161 */         target_abi,
/* FP:symbol.rs-2162 */         target_arch,
/* FP:symbol.rs-2163 */         target_endian,
/* FP:symbol.rs-2164 */         target_env,
/* FP:symbol.rs-2165 */         target_family,
/* FP:symbol.rs-2166 */         target_feature,
/* FP:symbol.rs-2167 */         target_feature_11,
/* FP:symbol.rs-2168 */         target_feature_inline_always,
/* FP:symbol.rs-2169 */         target_has_atomic,
/* FP:symbol.rs-2170 */         target_has_atomic_equal_alignment,
/* FP:symbol.rs-2171 */         target_has_atomic_load_store,
/* FP:symbol.rs-2172 */         target_has_reliable_f16,
/* FP:symbol.rs-2173 */         target_has_reliable_f16_math,
/* FP:symbol.rs-2174 */         target_has_reliable_f128,
/* FP:symbol.rs-2175 */         target_has_reliable_f128_math,
/* FP:symbol.rs-2176 */         target_os,
/* FP:symbol.rs-2177 */         target_pointer_width,
/* FP:symbol.rs-2178 */         target_thread_local,
/* FP:symbol.rs-2179 */         target_vendor,
/* FP:symbol.rs-2180 */         tbm_target_feature,
/* FP:symbol.rs-2181 */         termination,
/* FP:symbol.rs-2182 */         termination_trait,
/* FP:symbol.rs-2183 */         termination_trait_test,
/* FP:symbol.rs-2184 */         test,
/* FP:symbol.rs-2185 */         test_2018_feature,
/* FP:symbol.rs-2186 */         test_accepted_feature,
/* FP:symbol.rs-2187 */         test_case,
/* FP:symbol.rs-2188 */         test_removed_feature,
/* FP:symbol.rs-2189 */         test_runner,
/* FP:symbol.rs-2190 */         test_unstable_lint,
/* FP:symbol.rs-2191 */         thread,
/* FP:symbol.rs-2192 */         thread_local,
/* FP:symbol.rs-2193 */         thread_local_macro,
/* FP:symbol.rs-2194 */         three_way_compare,
/* FP:symbol.rs-2195 */         thumb2,
/* FP:symbol.rs-2196 */         thumb_mode: "thumb-mode",
/* FP:symbol.rs-2197 */         tmm_reg,
/* FP:symbol.rs-2198 */         to_owned_method,
/* FP:symbol.rs-2199 */         to_string,
/* FP:symbol.rs-2200 */         to_string_method,
/* FP:symbol.rs-2201 */         to_vec,
/* FP:symbol.rs-2202 */         todo_macro,
/* FP:symbol.rs-2203 */         tool_attributes,
/* FP:symbol.rs-2204 */         tool_lints,
/* FP:symbol.rs-2205 */         trace_macros,
/* FP:symbol.rs-2206 */         track_caller,
/* FP:symbol.rs-2207 */         trait_alias,
/* FP:symbol.rs-2208 */         trait_upcasting,
/* FP:symbol.rs-2209 */         transmute,
/* FP:symbol.rs-2210 */         transmute_generic_consts,
/* FP:symbol.rs-2211 */         transmute_opts,
/* FP:symbol.rs-2212 */         transmute_trait,
/* FP:symbol.rs-2213 */         transmute_unchecked,
/* FP:symbol.rs-2214 */         transparent,
/* FP:symbol.rs-2215 */         transparent_enums,
/* FP:symbol.rs-2216 */         transparent_unions,
/* FP:symbol.rs-2217 */         trivial_bounds,
/* FP:symbol.rs-2218 */         truncf16,
/* FP:symbol.rs-2219 */         truncf32,
/* FP:symbol.rs-2220 */         truncf64,
/* FP:symbol.rs-2221 */         truncf128,
/* FP:symbol.rs-2222 */         try_blocks,
/* FP:symbol.rs-2223 */         try_capture,
/* FP:symbol.rs-2224 */         try_from,
/* FP:symbol.rs-2225 */         try_from_fn,
/* FP:symbol.rs-2226 */         try_into,
/* FP:symbol.rs-2227 */         try_trait_v2,
/* FP:symbol.rs-2228 */         tt,
/* FP:symbol.rs-2229 */         tuple,
/* FP:symbol.rs-2230 */         tuple_indexing,
/* FP:symbol.rs-2231 */         tuple_trait,
/* FP:symbol.rs-2232 */         two_phase,
/* FP:symbol.rs-2233 */         ty,
/* FP:symbol.rs-2234 */         type_alias_enum_variants,
/* FP:symbol.rs-2235 */         type_alias_impl_trait,
/* FP:symbol.rs-2236 */         type_ascribe,
/* FP:symbol.rs-2237 */         type_ascription,
/* FP:symbol.rs-2238 */         type_changing_struct_update,
/* FP:symbol.rs-2239 */         type_const,
/* FP:symbol.rs-2240 */         type_id,
/* FP:symbol.rs-2241 */         type_id_eq,
/* FP:symbol.rs-2242 */         type_ir,
/* FP:symbol.rs-2243 */         type_ir_infer_ctxt_like,
/* FP:symbol.rs-2244 */         type_ir_inherent,
/* FP:symbol.rs-2245 */         type_ir_interner,
/* FP:symbol.rs-2246 */         type_length_limit,
/* FP:symbol.rs-2247 */         type_macros,
/* FP:symbol.rs-2248 */         type_name,
/* FP:symbol.rs-2249 */         type_privacy_lints,
/* FP:symbol.rs-2250 */         typed_swap_nonoverlapping,
/* FP:symbol.rs-2251 */         u8,
/* FP:symbol.rs-2252 */         u8_legacy_const_max,
/* FP:symbol.rs-2253 */         u8_legacy_const_min,
/* FP:symbol.rs-2254 */         u8_legacy_fn_max_value,
/* FP:symbol.rs-2255 */         u8_legacy_fn_min_value,
/* FP:symbol.rs-2256 */         u8_legacy_mod,
/* FP:symbol.rs-2257 */         u16,
/* FP:symbol.rs-2258 */         u16_legacy_const_max,
/* FP:symbol.rs-2259 */         u16_legacy_const_min,
/* FP:symbol.rs-2260 */         u16_legacy_fn_max_value,
/* FP:symbol.rs-2261 */         u16_legacy_fn_min_value,
/* FP:symbol.rs-2262 */         u16_legacy_mod,
/* FP:symbol.rs-2263 */         u32,
/* FP:symbol.rs-2264 */         u32_legacy_const_max,
/* FP:symbol.rs-2265 */         u32_legacy_const_min,
/* FP:symbol.rs-2266 */         u32_legacy_fn_max_value,
/* FP:symbol.rs-2267 */         u32_legacy_fn_min_value,
/* FP:symbol.rs-2268 */         u32_legacy_mod,
/* FP:symbol.rs-2269 */         u64,
/* FP:symbol.rs-2270 */         u64_legacy_const_max,
/* FP:symbol.rs-2271 */         u64_legacy_const_min,
/* FP:symbol.rs-2272 */         u64_legacy_fn_max_value,
/* FP:symbol.rs-2273 */         u64_legacy_fn_min_value,
/* FP:symbol.rs-2274 */         u64_legacy_mod,
/* FP:symbol.rs-2275 */         u128,
/* FP:symbol.rs-2276 */         u128_legacy_const_max,
/* FP:symbol.rs-2277 */         u128_legacy_const_min,
/* FP:symbol.rs-2278 */         u128_legacy_fn_max_value,
/* FP:symbol.rs-2279 */         u128_legacy_fn_min_value,
/* FP:symbol.rs-2280 */         u128_legacy_mod,
/* FP:symbol.rs-2281 */         ub_checks,
/* FP:symbol.rs-2282 */         unaligned_volatile_load,
/* FP:symbol.rs-2283 */         unaligned_volatile_store,
/* FP:symbol.rs-2284 */         unboxed_closures,
/* FP:symbol.rs-2285 */         unchecked_add,
/* FP:symbol.rs-2286 */         unchecked_div,
/* FP:symbol.rs-2287 */         unchecked_funnel_shl,
/* FP:symbol.rs-2288 */         unchecked_funnel_shr,
/* FP:symbol.rs-2289 */         unchecked_mul,
/* FP:symbol.rs-2290 */         unchecked_rem,
/* FP:symbol.rs-2291 */         unchecked_shl,
/* FP:symbol.rs-2292 */         unchecked_shr,
/* FP:symbol.rs-2293 */         unchecked_sub,
/* FP:symbol.rs-2294 */         undecorated,
/* FP:symbol.rs-2295 */         underscore_const_names,
/* FP:symbol.rs-2296 */         underscore_imports,
/* FP:symbol.rs-2297 */         underscore_lifetimes,
/* FP:symbol.rs-2298 */         uniform_paths,
/* FP:symbol.rs-2299 */         unimplemented_macro,
/* FP:symbol.rs-2300 */         unit,
/* FP:symbol.rs-2301 */         universal_impl_trait,
/* FP:symbol.rs-2302 */         unix,
/* FP:symbol.rs-2303 */         unlikely,
/* FP:symbol.rs-2304 */         unmarked_api,
/* FP:symbol.rs-2305 */         unnamed_fields,
/* FP:symbol.rs-2306 */         unpin,
/* FP:symbol.rs-2307 */         unqualified_local_imports,
/* FP:symbol.rs-2308 */         unreachable,
/* FP:symbol.rs-2309 */         unreachable_2015,
/* FP:symbol.rs-2310 */         unreachable_2015_macro,
/* FP:symbol.rs-2311 */         unreachable_2021,
/* FP:symbol.rs-2312 */         unreachable_code,
/* FP:symbol.rs-2313 */         unreachable_display,
/* FP:symbol.rs-2314 */         unreachable_macro,
/* FP:symbol.rs-2315 */         unrestricted_attribute_tokens,
/* FP:symbol.rs-2316 */         unsafe_attributes,
/* FP:symbol.rs-2317 */         unsafe_binders,
/* FP:symbol.rs-2318 */         unsafe_block_in_unsafe_fn,
/* FP:symbol.rs-2319 */         unsafe_cell,
/* FP:symbol.rs-2320 */         unsafe_cell_raw_get,
/* FP:symbol.rs-2321 */         unsafe_extern_blocks,
/* FP:symbol.rs-2322 */         unsafe_fields,
/* FP:symbol.rs-2323 */         unsafe_no_drop_flag,
/* FP:symbol.rs-2324 */         unsafe_pinned,
/* FP:symbol.rs-2325 */         unsafe_unpin,
/* FP:symbol.rs-2326 */         unsize,
/* FP:symbol.rs-2327 */         unsized_const_param_ty,
/* FP:symbol.rs-2328 */         unsized_const_params,
/* FP:symbol.rs-2329 */         unsized_fn_params,
/* FP:symbol.rs-2330 */         unsized_locals,
/* FP:symbol.rs-2331 */         unsized_tuple_coercion,
/* FP:symbol.rs-2332 */         unstable,
/* FP:symbol.rs-2333 */         unstable_feature_bound,
/* FP:symbol.rs-2334 */         unstable_location_reason_default: "this crate is being loaded from the sysroot, an \
/* FP:symbol.rs-2335 */                           unstable location; did you mean to load this crate \
/* FP:symbol.rs-2336 */                           from crates.io via `Cargo.toml` instead?",
/* FP:symbol.rs-2337 */         untagged_unions,
/* FP:symbol.rs-2338 */         unused_imports,
/* FP:symbol.rs-2339 */         unwind,
/* FP:symbol.rs-2340 */         unwind_attributes,
/* FP:symbol.rs-2341 */         unwind_safe_trait,
/* FP:symbol.rs-2342 */         unwrap,
/* FP:symbol.rs-2343 */         unwrap_binder,
/* FP:symbol.rs-2344 */         unwrap_or,
/* FP:symbol.rs-2345 */         use_cloned,
/* FP:symbol.rs-2346 */         use_extern_macros,
/* FP:symbol.rs-2347 */         use_nested_groups,
/* FP:symbol.rs-2348 */         used,
/* FP:symbol.rs-2349 */         used_with_arg,
/* FP:symbol.rs-2350 */         using,
/* FP:symbol.rs-2351 */         usize,
/* FP:symbol.rs-2352 */         usize_legacy_const_max,
/* FP:symbol.rs-2353 */         usize_legacy_const_min,
/* FP:symbol.rs-2354 */         usize_legacy_fn_max_value,
/* FP:symbol.rs-2355 */         usize_legacy_fn_min_value,
/* FP:symbol.rs-2356 */         usize_legacy_mod,
/* FP:symbol.rs-2357 */         v1,
/* FP:symbol.rs-2358 */         v8plus,
/* FP:symbol.rs-2359 */         va_arg,
/* FP:symbol.rs-2360 */         va_copy,
/* FP:symbol.rs-2361 */         va_end,
/* FP:symbol.rs-2362 */         va_list,
/* FP:symbol.rs-2363 */         va_start,
/* FP:symbol.rs-2364 */         val,
/* FP:symbol.rs-2365 */         validity,
/* FP:symbol.rs-2366 */         value,
/* FP:symbol.rs-2367 */         values,
/* FP:symbol.rs-2368 */         var,
/* FP:symbol.rs-2369 */         variant_count,
/* FP:symbol.rs-2370 */         vec,
/* FP:symbol.rs-2371 */         vec_as_mut_slice,
/* FP:symbol.rs-2372 */         vec_as_slice,
/* FP:symbol.rs-2373 */         vec_from_elem,
/* FP:symbol.rs-2374 */         vec_is_empty,
/* FP:symbol.rs-2375 */         vec_macro,
/* FP:symbol.rs-2376 */         vec_new,
/* FP:symbol.rs-2377 */         vec_pop,
/* FP:symbol.rs-2378 */         vec_reserve,
/* FP:symbol.rs-2379 */         vec_with_capacity,
/* FP:symbol.rs-2380 */         vecdeque_iter,
/* FP:symbol.rs-2381 */         vecdeque_reserve,
/* FP:symbol.rs-2382 */         vector,
/* FP:symbol.rs-2383 */         verbatim,
/* FP:symbol.rs-2384 */         version,
/* FP:symbol.rs-2385 */         vfp2,
/* FP:symbol.rs-2386 */         vis,
/* FP:symbol.rs-2387 */         visible_private_types,
/* FP:symbol.rs-2388 */         volatile,
/* FP:symbol.rs-2389 */         volatile_copy_memory,
/* FP:symbol.rs-2390 */         volatile_copy_nonoverlapping_memory,
/* FP:symbol.rs-2391 */         volatile_load,
/* FP:symbol.rs-2392 */         volatile_set_memory,
/* FP:symbol.rs-2393 */         volatile_store,
/* FP:symbol.rs-2394 */         vreg,
/* FP:symbol.rs-2395 */         vreg_low16,
/* FP:symbol.rs-2396 */         vsx,
/* FP:symbol.rs-2397 */         vtable_align,
/* FP:symbol.rs-2398 */         vtable_size,
/* FP:symbol.rs-2399 */         warn,
/* FP:symbol.rs-2400 */         wasip2,
/* FP:symbol.rs-2401 */         wasm_abi,
/* FP:symbol.rs-2402 */         wasm_import_module,
/* FP:symbol.rs-2403 */         wasm_target_feature,
/* FP:symbol.rs-2404 */         weak,
/* FP:symbol.rs-2405 */         weak_odr,
/* FP:symbol.rs-2406 */         where_clause_attrs,
/* FP:symbol.rs-2407 */         while_let,
/* FP:symbol.rs-2408 */         whole_dash_archive: "whole-archive",
/* FP:symbol.rs-2409 */         width,
/* FP:symbol.rs-2410 */         windows,
/* FP:symbol.rs-2411 */         windows_subsystem,
/* FP:symbol.rs-2412 */         with_negative_coherence,
/* FP:symbol.rs-2413 */         wrap_binder,
/* FP:symbol.rs-2414 */         wrapping_add,
/* FP:symbol.rs-2415 */         wrapping_div,
/* FP:symbol.rs-2416 */         wrapping_mul,
/* FP:symbol.rs-2417 */         wrapping_rem,
/* FP:symbol.rs-2418 */         wrapping_rem_euclid,
/* FP:symbol.rs-2419 */         wrapping_sub,
/* FP:symbol.rs-2420 */         wreg,
/* FP:symbol.rs-2421 */         write_bytes,
/* FP:symbol.rs-2422 */         write_fmt,
/* FP:symbol.rs-2423 */         write_macro,
/* FP:symbol.rs-2424 */         write_str,
/* FP:symbol.rs-2425 */         write_via_move,
/* FP:symbol.rs-2426 */         writeln_macro,
/* FP:symbol.rs-2427 */         x86_amx_intrinsics,
/* FP:symbol.rs-2428 */         x87_reg,
/* FP:symbol.rs-2429 */         x87_target_feature,
/* FP:symbol.rs-2430 */         xer,
/* FP:symbol.rs-2431 */         xmm_reg,
/* FP:symbol.rs-2432 */         xop_target_feature,
/* FP:symbol.rs-2433 */         yeet_desugar_details,
/* FP:symbol.rs-2434 */         yeet_expr,
/* FP:symbol.rs-2435 */         yes,
/* FP:symbol.rs-2436 */         yield_expr,
/* FP:symbol.rs-2437 */         ymm_reg,
/* FP:symbol.rs-2438 */         yreg,
/* FP:symbol.rs-2439 */         zca,
/* FP:symbol.rs-2440 */         zfh,
/* FP:symbol.rs-2441 */         zfhmin,
/* FP:symbol.rs-2442 */         zmm_reg,
/* FP:symbol.rs-2443 */         ztso,
/* FP:symbol.rs-2444 */         // tidy-alphabetical-end
/* FP:symbol.rs-2445 */     }
/* FP:symbol.rs-2446 */ }
/* FP:symbol.rs-2447 */ 
/* FP:symbol.rs-2448 */ /// Symbols for crates that are part of the stable standard library: `std`, `core`, `alloc`, and
/* FP:symbol.rs-2449 */ /// `proc_macro`.
/* FP:symbol.rs-2450 */ pub const STDLIB_STABLE_CRATES: &[Symbol] = &[sym::std, sym::core, sym::alloc, sym::proc_macro];
/* FP:symbol.rs-2451 */ 
/* FP:symbol.rs-2452 */ #[derive(Copy, Clone, Eq, HashStable_Generic, Encodable, Decodable)]
/* FP:symbol.rs-2453 */ pub struct Ident {
/* FP:symbol.rs-2454 */     // `name` should never be the empty symbol. If you are considering that,
/* FP:symbol.rs-2455 */     // you are probably conflating "empty identifier with "no identifier" and
/* FP:symbol.rs-2456 */     // you should use `Option<Ident>` instead.
/* FP:symbol.rs-2457 */     pub name: Symbol,
/* FP:symbol.rs-2458 */     pub span: Span,
/* FP:symbol.rs-2459 */ }
/* FP:symbol.rs-2460 */ 
/* FP:symbol.rs-2461 */ impl Ident {
/* FP:symbol.rs-2462 */     #[inline]
/* FP:symbol.rs-2463 */     /// Constructs a new identifier from a symbol and a span.
/* FP:symbol.rs-2464 */     pub fn new(name: Symbol, span: Span) -> Ident {
/* FP:symbol.rs-2465 */         debug_assert_ne!(name, sym::empty);
/* FP:symbol.rs-2466 */         Ident { name, span }
/* FP:symbol.rs-2467 */     }
/* FP:symbol.rs-2468 */ 
/* FP:symbol.rs-2469 */     /// Constructs a new identifier with a dummy span.
/* FP:symbol.rs-2470 */     #[inline]
/* FP:symbol.rs-2471 */     pub fn with_dummy_span(name: Symbol) -> Ident {
/* FP:symbol.rs-2472 */         Ident::new(name, DUMMY_SP)
/* FP:symbol.rs-2473 */     }
/* FP:symbol.rs-2474 */ 
/* FP:symbol.rs-2475 */     // For dummy identifiers that are never used and absolutely must be
/* FP:symbol.rs-2476 */     // present. Note that this does *not* use the empty symbol; `sym::dummy`
/* FP:symbol.rs-2477 */     // makes it clear that it's intended as a dummy value, and is more likely
/* FP:symbol.rs-2478 */     // to be detected if it accidentally does get used.
/* FP:symbol.rs-2479 */     #[inline]
/* FP:symbol.rs-2480 */     pub fn dummy() -> Ident {
/* FP:symbol.rs-2481 */         Ident::with_dummy_span(sym::dummy)
/* FP:symbol.rs-2482 */     }
/* FP:symbol.rs-2483 */ 
/* FP:symbol.rs-2484 */     /// Maps a string to an identifier with a dummy span.
/* FP:symbol.rs-2485 */     pub fn from_str(string: &str) -> Ident {
/* FP:symbol.rs-2486 */         Ident::with_dummy_span(Symbol::intern(string))
/* FP:symbol.rs-2487 */     }
/* FP:symbol.rs-2488 */ 
/* FP:symbol.rs-2489 */     /// Maps a string and a span to an identifier.
/* FP:symbol.rs-2490 */     pub fn from_str_and_span(string: &str, span: Span) -> Ident {
/* FP:symbol.rs-2491 */         Ident::new(Symbol::intern(string), span)
/* FP:symbol.rs-2492 */     }
/* FP:symbol.rs-2493 */ 
/* FP:symbol.rs-2494 */     /// Replaces `lo` and `hi` with those from `span`, but keep hygiene context.
/* FP:symbol.rs-2495 */     pub fn with_span_pos(self, span: Span) -> Ident {
/* FP:symbol.rs-2496 */         Ident::new(self.name, span.with_ctxt(self.span.ctxt()))
/* FP:symbol.rs-2497 */     }
/* FP:symbol.rs-2498 */ 
/* FP:symbol.rs-2499 */     pub fn without_first_quote(self) -> Ident {
/* FP:symbol.rs-2500 */         Ident::new(Symbol::intern(self.as_str().trim_start_matches('\'')), self.span)
/* FP:symbol.rs-2501 */     }
/* FP:symbol.rs-2502 */ 
/* FP:symbol.rs-2503 */     /// "Normalize" ident for use in comparisons using "item hygiene".
/* FP:symbol.rs-2504 */     /// Identifiers with same string value become same if they came from the same macro 2.0 macro
/* FP:symbol.rs-2505 */     /// (e.g., `macro` item, but not `macro_rules` item) and stay different if they came from
/* FP:symbol.rs-2506 */     /// different macro 2.0 macros.
/* FP:symbol.rs-2507 */     /// Technically, this operation strips all non-opaque marks from ident's syntactic context.
/* FP:symbol.rs-2508 */     pub fn normalize_to_macros_2_0(self) -> Ident {
/* FP:symbol.rs-2509 */         Ident::new(self.name, self.span.normalize_to_macros_2_0())
/* FP:symbol.rs-2510 */     }
/* FP:symbol.rs-2511 */ 
/* FP:symbol.rs-2512 */     /// "Normalize" ident for use in comparisons using "local variable hygiene".
/* FP:symbol.rs-2513 */     /// Identifiers with same string value become same if they came from the same non-transparent
/* FP:symbol.rs-2514 */     /// macro (e.g., `macro` or `macro_rules!` items) and stay different if they came from different
/* FP:symbol.rs-2515 */     /// non-transparent macros.
/* FP:symbol.rs-2516 */     /// Technically, this operation strips all transparent marks from ident's syntactic context.
/* FP:symbol.rs-2517 */     #[inline]
/* FP:symbol.rs-2518 */     pub fn normalize_to_macro_rules(self) -> Ident {
/* FP:symbol.rs-2519 */         Ident::new(self.name, self.span.normalize_to_macro_rules())
/* FP:symbol.rs-2520 */     }
/* FP:symbol.rs-2521 */ 
/* FP:symbol.rs-2522 */     /// Access the underlying string. This is a slowish operation because it
/* FP:symbol.rs-2523 */     /// requires locking the symbol interner.
/* FP:symbol.rs-2524 */     ///
/* FP:symbol.rs-2525 */     /// Note that the lifetime of the return value is a lie. See
/* FP:symbol.rs-2526 */     /// `Symbol::as_str()` for details.
/* FP:symbol.rs-2527 */     pub fn as_str(&self) -> &str {
/* FP:symbol.rs-2528 */         self.name.as_str()
/* FP:symbol.rs-2529 */     }
/* FP:symbol.rs-2530 */ }
/* FP:symbol.rs-2531 */ 
/* FP:symbol.rs-2532 */ impl PartialEq for Ident {
/* FP:symbol.rs-2533 */     #[inline]
/* FP:symbol.rs-2534 */     fn eq(&self, rhs: &Self) -> bool {
/* FP:symbol.rs-2535 */         self.name == rhs.name && self.span.eq_ctxt(rhs.span)
/* FP:symbol.rs-2536 */     }
/* FP:symbol.rs-2537 */ }
/* FP:symbol.rs-2538 */ 
/* FP:symbol.rs-2539 */ impl Hash for Ident {
/* FP:symbol.rs-2540 */     fn hash<H: Hasher>(&self, state: &mut H) {
/* FP:symbol.rs-2541 */         self.name.hash(state);
/* FP:symbol.rs-2542 */         self.span.ctxt().hash(state);
/* FP:symbol.rs-2543 */     }
/* FP:symbol.rs-2544 */ }
/* FP:symbol.rs-2545 */ 
/* FP:symbol.rs-2546 */ impl fmt::Debug for Ident {
/* FP:symbol.rs-2547 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2548 */         fmt::Display::fmt(self, f)?;
/* FP:symbol.rs-2549 */         fmt::Debug::fmt(&self.span.ctxt(), f)
/* FP:symbol.rs-2550 */     }
/* FP:symbol.rs-2551 */ }
/* FP:symbol.rs-2552 */ 
/* FP:symbol.rs-2553 */ /// This implementation is supposed to be used in error messages, so it's expected to be identical
/* FP:symbol.rs-2554 */ /// to printing the original identifier token written in source code (`token_to_string`),
/* FP:symbol.rs-2555 */ /// except that AST identifiers don't keep the rawness flag, so we have to guess it.
/* FP:symbol.rs-2556 */ impl fmt::Display for Ident {
/* FP:symbol.rs-2557 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2558 */         fmt::Display::fmt(&IdentPrinter::new(self.name, self.guess_print_mode(), None), f)
/* FP:symbol.rs-2559 */     }
/* FP:symbol.rs-2560 */ }
/* FP:symbol.rs-2561 */ 
/* FP:symbol.rs-2562 */ pub enum IdentPrintMode {
/* FP:symbol.rs-2563 */     Normal,
/* FP:symbol.rs-2564 */     RawIdent,
/* FP:symbol.rs-2565 */     RawLifetime,
/* FP:symbol.rs-2566 */ }
/* FP:symbol.rs-2567 */ 
/* FP:symbol.rs-2568 */ /// The most general type to print identifiers.
/* FP:symbol.rs-2569 */ ///
/* FP:symbol.rs-2570 */ /// AST pretty-printer is used as a fallback for turning AST structures into token streams for
/* FP:symbol.rs-2571 */ /// proc macros. Additionally, proc macros may stringify their input and expect it survive the
/* FP:symbol.rs-2572 */ /// stringification (especially true for proc macro derives written between Rust 1.15 and 1.30).
/* FP:symbol.rs-2573 */ /// So we need to somehow pretty-print `$crate` in a way preserving at least some of its
/* FP:symbol.rs-2574 */ /// hygiene data, most importantly name of the crate it refers to.
/* FP:symbol.rs-2575 */ /// As a result we print `$crate` as `crate` if it refers to the local crate
/* FP:symbol.rs-2576 */ /// and as `::other_crate_name` if it refers to some other crate.
/* FP:symbol.rs-2577 */ /// Note, that this is only done if the ident token is printed from inside of AST pretty-printing,
/* FP:symbol.rs-2578 */ /// but not otherwise. Pretty-printing is the only way for proc macros to discover token contents,
/* FP:symbol.rs-2579 */ /// so we should not perform this lossy conversion if the top level call to the pretty-printer was
/* FP:symbol.rs-2580 */ /// done for a token stream or a single token.
/* FP:symbol.rs-2581 */ pub struct IdentPrinter {
/* FP:symbol.rs-2582 */     symbol: Symbol,
/* FP:symbol.rs-2583 */     mode: IdentPrintMode,
/* FP:symbol.rs-2584 */     /// Span used for retrieving the crate name to which `$crate` refers to,
/* FP:symbol.rs-2585 */     /// if this field is `None` then the `$crate` conversion doesn't happen.
/* FP:symbol.rs-2586 */     convert_dollar_crate: Option<Span>,
/* FP:symbol.rs-2587 */ }
/* FP:symbol.rs-2588 */ 
/* FP:symbol.rs-2589 */ impl IdentPrinter {
/* FP:symbol.rs-2590 */     /// The most general `IdentPrinter` constructor. Do not use this.
/* FP:symbol.rs-2591 */     pub fn new(
/* FP:symbol.rs-2592 */         symbol: Symbol,
/* FP:symbol.rs-2593 */         mode: IdentPrintMode,
/* FP:symbol.rs-2594 */         convert_dollar_crate: Option<Span>,
/* FP:symbol.rs-2595 */     ) -> IdentPrinter {
/* FP:symbol.rs-2596 */         IdentPrinter { symbol, mode, convert_dollar_crate }
/* FP:symbol.rs-2597 */     }
/* FP:symbol.rs-2598 */ 
/* FP:symbol.rs-2599 */     /// This implementation is supposed to be used when printing identifiers
/* FP:symbol.rs-2600 */     /// as a part of pretty-printing for larger AST pieces.
/* FP:symbol.rs-2601 */     /// Do not use this either.
/* FP:symbol.rs-2602 */     pub fn for_ast_ident(ident: Ident, mode: IdentPrintMode) -> IdentPrinter {
/* FP:symbol.rs-2603 */         IdentPrinter::new(ident.name, mode, Some(ident.span))
/* FP:symbol.rs-2604 */     }
/* FP:symbol.rs-2605 */ }
/* FP:symbol.rs-2606 */ 
/* FP:symbol.rs-2607 */ impl fmt::Display for IdentPrinter {
/* FP:symbol.rs-2608 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2609 */         let s = match self.mode {
/* FP:symbol.rs-2610 */             IdentPrintMode::Normal
/* FP:symbol.rs-2611 */                 if self.symbol == kw::DollarCrate
/* FP:symbol.rs-2612 */                     && let Some(span) = self.convert_dollar_crate =>
/* FP:symbol.rs-2613 */             {
/* FP:symbol.rs-2614 */                 let converted = span.ctxt().dollar_crate_name();
/* FP:symbol.rs-2615 */                 if !converted.is_path_segment_keyword() {
/* FP:symbol.rs-2616 */                     f.write_str("::")?;
/* FP:symbol.rs-2617 */                 }
/* FP:symbol.rs-2618 */                 converted
/* FP:symbol.rs-2619 */             }
/* FP:symbol.rs-2620 */             IdentPrintMode::Normal => self.symbol,
/* FP:symbol.rs-2621 */             IdentPrintMode::RawIdent => {
/* FP:symbol.rs-2622 */                 f.write_str("r#")?;
/* FP:symbol.rs-2623 */                 self.symbol
/* FP:symbol.rs-2624 */             }
/* FP:symbol.rs-2625 */             IdentPrintMode::RawLifetime => {
/* FP:symbol.rs-2626 */                 f.write_str("'r#")?;
/* FP:symbol.rs-2627 */                 let s = self
/* FP:symbol.rs-2628 */                     .symbol
/* FP:symbol.rs-2629 */                     .as_str()
/* FP:symbol.rs-2630 */                     .strip_prefix("'")
/* FP:symbol.rs-2631 */                     .expect("only lifetime idents should be passed with RawLifetime mode");
/* FP:symbol.rs-2632 */                 Symbol::intern(s)
/* FP:symbol.rs-2633 */             }
/* FP:symbol.rs-2634 */         };
/* FP:symbol.rs-2635 */         s.fmt(f)
/* FP:symbol.rs-2636 */     }
/* FP:symbol.rs-2637 */ }
/* FP:symbol.rs-2638 */ 
/* FP:symbol.rs-2639 */ /// An newtype around `Ident` that calls [Ident::normalize_to_macro_rules] on
/* FP:symbol.rs-2640 */ /// construction for "local variable hygiene" comparisons.
/* FP:symbol.rs-2641 */ ///
/* FP:symbol.rs-2642 */ /// Use this type when you need to compare identifiers according to macro_rules hygiene.
/* FP:symbol.rs-2643 */ /// This ensures compile-time safety and avoids manual normalization calls.
/* FP:symbol.rs-2644 */ #[derive(Copy, Clone, Eq, PartialEq, Hash)]
/* FP:symbol.rs-2645 */ pub struct MacroRulesNormalizedIdent(Ident);
/* FP:symbol.rs-2646 */ 
/* FP:symbol.rs-2647 */ impl MacroRulesNormalizedIdent {
/* FP:symbol.rs-2648 */     #[inline]
/* FP:symbol.rs-2649 */     pub fn new(ident: Ident) -> Self {
/* FP:symbol.rs-2650 */         MacroRulesNormalizedIdent(ident.normalize_to_macro_rules())
/* FP:symbol.rs-2651 */     }
/* FP:symbol.rs-2652 */ }
/* FP:symbol.rs-2653 */ 
/* FP:symbol.rs-2654 */ impl fmt::Debug for MacroRulesNormalizedIdent {
/* FP:symbol.rs-2655 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2656 */         fmt::Debug::fmt(&self.0, f)
/* FP:symbol.rs-2657 */     }
/* FP:symbol.rs-2658 */ }
/* FP:symbol.rs-2659 */ 
/* FP:symbol.rs-2660 */ impl fmt::Display for MacroRulesNormalizedIdent {
/* FP:symbol.rs-2661 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2662 */         fmt::Display::fmt(&self.0, f)
/* FP:symbol.rs-2663 */     }
/* FP:symbol.rs-2664 */ }
/* FP:symbol.rs-2665 */ 
/* FP:symbol.rs-2666 */ /// An newtype around `Ident` that calls [Ident::normalize_to_macros_2_0] on
/* FP:symbol.rs-2667 */ /// construction for "item hygiene" comparisons.
/* FP:symbol.rs-2668 */ ///
/* FP:symbol.rs-2669 */ /// Identifiers with same string value become same if they came from the same macro 2.0 macro
/* FP:symbol.rs-2670 */ /// (e.g., `macro` item, but not `macro_rules` item) and stay different if they came from
/* FP:symbol.rs-2671 */ /// different macro 2.0 macros.
/* FP:symbol.rs-2672 */ #[derive(Copy, Clone, Eq, PartialEq, Hash)]
/* FP:symbol.rs-2673 */ pub struct Macros20NormalizedIdent(pub Ident);
/* FP:symbol.rs-2674 */ 
/* FP:symbol.rs-2675 */ impl Macros20NormalizedIdent {
/* FP:symbol.rs-2676 */     #[inline]
/* FP:symbol.rs-2677 */     pub fn new(ident: Ident) -> Self {
/* FP:symbol.rs-2678 */         Macros20NormalizedIdent(ident.normalize_to_macros_2_0())
/* FP:symbol.rs-2679 */     }
/* FP:symbol.rs-2680 */ 
/* FP:symbol.rs-2681 */     // dummy_span does not need to be normalized, so we can use `Ident` directly
/* FP:symbol.rs-2682 */     pub fn with_dummy_span(name: Symbol) -> Self {
/* FP:symbol.rs-2683 */         Macros20NormalizedIdent(Ident::with_dummy_span(name))
/* FP:symbol.rs-2684 */     }
/* FP:symbol.rs-2685 */ }
/* FP:symbol.rs-2686 */ 
/* FP:symbol.rs-2687 */ impl fmt::Debug for Macros20NormalizedIdent {
/* FP:symbol.rs-2688 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2689 */         fmt::Debug::fmt(&self.0, f)
/* FP:symbol.rs-2690 */     }
/* FP:symbol.rs-2691 */ }
/* FP:symbol.rs-2692 */ 
/* FP:symbol.rs-2693 */ impl fmt::Display for Macros20NormalizedIdent {
/* FP:symbol.rs-2694 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2695 */         fmt::Display::fmt(&self.0, f)
/* FP:symbol.rs-2696 */     }
/* FP:symbol.rs-2697 */ }
/* FP:symbol.rs-2698 */ 
/* FP:symbol.rs-2699 */ /// By impl Deref, we can access the wrapped Ident as if it were a normal Ident
/* FP:symbol.rs-2700 */ /// such as `norm_ident.name` instead of `norm_ident.0.name`.
/* FP:symbol.rs-2701 */ impl Deref for Macros20NormalizedIdent {
/* FP:symbol.rs-2702 */     type Target = Ident;
/* FP:symbol.rs-2703 */     fn deref(&self) -> &Self::Target {
/* FP:symbol.rs-2704 */         &self.0
/* FP:symbol.rs-2705 */     }
/* FP:symbol.rs-2706 */ }
/* FP:symbol.rs-2707 */ 
/* FP:symbol.rs-2708 */ /// An interned UTF-8 string.
/* FP:symbol.rs-2709 */ ///
/* FP:symbol.rs-2710 */ /// Internally, a `Symbol` is implemented as an index, and all operations
/* FP:symbol.rs-2711 */ /// (including hashing, equality, and ordering) operate on that index. The use
/* FP:symbol.rs-2712 */ /// of `crate::rustc_index::newtype_index!` means that `Option<Symbol>` only takes up 4 bytes,
/* FP:symbol.rs-2713 */ /// because `crate::rustc_index::newtype_index!` reserves the last 256 values for tagging purposes.
/* FP:symbol.rs-2714 */ ///
/* FP:symbol.rs-2715 */ /// Note that `Symbol` cannot directly be a `crate::rustc_index::newtype_index!` because it
/* FP:symbol.rs-2716 */ /// implements `fmt::Debug`, `Encodable`, and `Decodable` in special ways.
/* FP:symbol.rs-2717 */ #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* FP:symbol.rs-2718 */ pub struct Symbol(SymbolIndex);
/* FP:symbol.rs-2719 */ 
/* FP:symbol.rs-2720 */ // Used within both `Symbol` and `ByteSymbol`.
/* FP:symbol.rs-2721 */ crate::rustc_index::newtype_index! {
/* FP:symbol.rs-2722 */     #[orderable]
/* FP:symbol.rs-2723 */     struct SymbolIndex {}
/* FP:symbol.rs-2724 */ }
/* FP:symbol.rs-2725 */ 
/* FP:symbol.rs-2726 */ impl Symbol {
/* FP:symbol.rs-2727 */     /// Avoid this except for things like deserialization of previously
/* FP:symbol.rs-2728 */     /// serialized symbols, and testing. Use `intern` instead.
/* FP:symbol.rs-2729 */     pub const fn new(n: u32) -> Self {
/* FP:symbol.rs-2730 */         Symbol(SymbolIndex::from_u32(n))
/* FP:symbol.rs-2731 */     }
/* FP:symbol.rs-2732 */ 
/* FP:symbol.rs-2733 */     /// Maps a string to its interned representation.
/* FP:symbol.rs-2734 */     #[rustc_diagnostic_item = "SymbolIntern"]
/* FP:symbol.rs-2735 */     pub fn intern(str: &str) -> Self {
/* FP:symbol.rs-2736 */         with_session_globals(|session_globals| session_globals.symbol_interner.intern_str(str))
/* FP:symbol.rs-2737 */     }
/* FP:symbol.rs-2738 */ 
/* FP:symbol.rs-2739 */     /// Access the underlying string. This is a slowish operation because it
/* FP:symbol.rs-2740 */     /// requires locking the symbol interner.
/* FP:symbol.rs-2741 */     ///
/* FP:symbol.rs-2742 */     /// Note that the lifetime of the return value is a lie. It's not the same
/* FP:symbol.rs-2743 */     /// as `&self`, but actually tied to the lifetime of the underlying
/* FP:symbol.rs-2744 */     /// interner. Interners are long-lived, and there are very few of them, and
/* FP:symbol.rs-2745 */     /// this function is typically used for short-lived things, so in practice
/* FP:symbol.rs-2746 */     /// it works out ok.
/* FP:symbol.rs-2747 */     pub fn as_str(&self) -> &str {
/* FP:symbol.rs-2748 */         with_session_globals(|session_globals| unsafe {
/* FP:symbol.rs-2749 */             std::mem::transmute::<&str, &str>(session_globals.symbol_interner.get_str(*self))
/* FP:symbol.rs-2750 */         })
/* FP:symbol.rs-2751 */     }
/* FP:symbol.rs-2752 */ 
/* FP:symbol.rs-2753 */     pub fn as_u32(self) -> u32 {
/* FP:symbol.rs-2754 */         self.0.as_u32()
/* FP:symbol.rs-2755 */     }
/* FP:symbol.rs-2756 */ 
/* FP:symbol.rs-2757 */     pub fn is_empty(self) -> bool {
/* FP:symbol.rs-2758 */         self == sym::empty
/* FP:symbol.rs-2759 */     }
/* FP:symbol.rs-2760 */ 
/* FP:symbol.rs-2761 */     /// This method is supposed to be used in error messages, so it's expected to be
/* FP:symbol.rs-2762 */     /// identical to printing the original identifier token written in source code
/* FP:symbol.rs-2763 */     /// (`token_to_string`, `Ident::to_string`), except that symbols don't keep the rawness flag
/* FP:symbol.rs-2764 */     /// or edition, so we have to guess the rawness using the global edition.
/* FP:symbol.rs-2765 */     pub fn to_ident_string(self) -> String {
/* FP:symbol.rs-2766 */         // Avoid creating an empty identifier, because that asserts in debug builds.
/* FP:symbol.rs-2767 */         if self == sym::empty { String::new() } else { Ident::with_dummy_span(self).to_string() }
/* FP:symbol.rs-2768 */     }
/* FP:symbol.rs-2769 */ }
/* FP:symbol.rs-2770 */ 
/* FP:symbol.rs-2771 */ impl fmt::Debug for Symbol {
/* FP:symbol.rs-2772 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2773 */         fmt::Debug::fmt(self.as_str(), f)
/* FP:symbol.rs-2774 */     }
/* FP:symbol.rs-2775 */ }
/* FP:symbol.rs-2776 */ 
/* FP:symbol.rs-2777 */ impl fmt::Display for Symbol {
/* FP:symbol.rs-2778 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2779 */         fmt::Display::fmt(self.as_str(), f)
/* FP:symbol.rs-2780 */     }
/* FP:symbol.rs-2781 */ }
/* FP:symbol.rs-2782 */ 
/* FP:symbol.rs-2783 */ impl<CTX> HashStable<CTX> for Symbol {
/* FP:symbol.rs-2784 */     #[inline]
/* FP:symbol.rs-2785 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
/* FP:symbol.rs-2786 */         self.as_str().hash_stable(hcx, hasher);
/* FP:symbol.rs-2787 */     }
/* FP:symbol.rs-2788 */ }
/* FP:symbol.rs-2789 */ 
/* FP:symbol.rs-2790 */ impl<CTX> ToStableHashKey<CTX> for Symbol {
/* FP:symbol.rs-2791 */     type KeyType = String;
/* FP:symbol.rs-2792 */     #[inline]
/* FP:symbol.rs-2793 */     fn to_stable_hash_key(&self, _: &CTX) -> String {
/* FP:symbol.rs-2794 */         self.as_str().to_string()
/* FP:symbol.rs-2795 */     }
/* FP:symbol.rs-2796 */ }
/* FP:symbol.rs-2797 */ 
/* FP:symbol.rs-2798 */ impl StableCompare for Symbol {
/* FP:symbol.rs-2799 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:symbol.rs-2800 */ 
/* FP:symbol.rs-2801 */     fn stable_cmp(&self, other: &Self) -> std::cmp::Ordering {
/* FP:symbol.rs-2802 */         self.as_str().cmp(other.as_str())
/* FP:symbol.rs-2803 */     }
/* FP:symbol.rs-2804 */ }
/* FP:symbol.rs-2805 */ 
/* FP:symbol.rs-2806 */ /// Like `Symbol`, but for byte strings. `ByteSymbol` is used less widely, so
/* FP:symbol.rs-2807 */ /// it has fewer operations defined than `Symbol`.
/* FP:symbol.rs-2808 */ #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* FP:symbol.rs-2809 */ pub struct ByteSymbol(SymbolIndex);
/* FP:symbol.rs-2810 */ 
/* FP:symbol.rs-2811 */ impl ByteSymbol {
/* FP:symbol.rs-2812 */     /// Avoid this except for things like deserialization of previously
/* FP:symbol.rs-2813 */     /// serialized symbols, and testing. Use `intern` instead.
/* FP:symbol.rs-2814 */     pub const fn new(n: u32) -> Self {
/* FP:symbol.rs-2815 */         ByteSymbol(SymbolIndex::from_u32(n))
/* FP:symbol.rs-2816 */     }
/* FP:symbol.rs-2817 */ 
/* FP:symbol.rs-2818 */     /// Maps a string to its interned representation.
/* FP:symbol.rs-2819 */     pub fn intern(byte_str: &[u8]) -> Self {
/* FP:symbol.rs-2820 */         with_session_globals(|session_globals| {
/* FP:symbol.rs-2821 */             session_globals.symbol_interner.intern_byte_str(byte_str)
/* FP:symbol.rs-2822 */         })
/* FP:symbol.rs-2823 */     }
/* FP:symbol.rs-2824 */ 
/* FP:symbol.rs-2825 */     /// Like `Symbol::as_str`.
/* FP:symbol.rs-2826 */     pub fn as_byte_str(&self) -> &[u8] {
/* FP:symbol.rs-2827 */         with_session_globals(|session_globals| unsafe {
/* FP:symbol.rs-2828 */             std::mem::transmute::<&[u8], &[u8]>(session_globals.symbol_interner.get_byte_str(*self))
/* FP:symbol.rs-2829 */         })
/* FP:symbol.rs-2830 */     }
/* FP:symbol.rs-2831 */ 
/* FP:symbol.rs-2832 */     pub fn as_u32(self) -> u32 {
/* FP:symbol.rs-2833 */         self.0.as_u32()
/* FP:symbol.rs-2834 */     }
/* FP:symbol.rs-2835 */ }
/* FP:symbol.rs-2836 */ 
/* FP:symbol.rs-2837 */ impl fmt::Debug for ByteSymbol {
/* FP:symbol.rs-2838 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:symbol.rs-2839 */         fmt::Debug::fmt(self.as_byte_str(), f)
/* FP:symbol.rs-2840 */     }
/* FP:symbol.rs-2841 */ }
/* FP:symbol.rs-2842 */ 
/* FP:symbol.rs-2843 */ impl<CTX> HashStable<CTX> for ByteSymbol {
/* FP:symbol.rs-2844 */     #[inline]
/* FP:symbol.rs-2845 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
/* FP:symbol.rs-2846 */         self.as_byte_str().hash_stable(hcx, hasher);
/* FP:symbol.rs-2847 */     }
/* FP:symbol.rs-2848 */ }
/* FP:symbol.rs-2849 */ 
/* FP:symbol.rs-2850 */ // Interner used for both `Symbol`s and `ByteSymbol`s. If a string and a byte
/* FP:symbol.rs-2851 */ // string with identical contents (e.g. "foo" and b"foo") are both interned,
/* FP:symbol.rs-2852 */ // only one copy will be stored and the resulting `Symbol` and `ByteSymbol`
/* FP:symbol.rs-2853 */ // will have the same index.
/* FP:symbol.rs-2854 */ pub(crate) struct Interner(Lock<InternerInner>);
/* FP:symbol.rs-2855 */ 
/* FP:symbol.rs-2856 */ // The `&'static [u8]`s in this type actually point into the arena.
/* FP:symbol.rs-2857 */ //
/* FP:symbol.rs-2858 */ // This type is private to prevent accidentally constructing more than one
/* FP:symbol.rs-2859 */ // `Interner` on the same thread, which makes it easy to mix up `Symbol`s
/* FP:symbol.rs-2860 */ // between `Interner`s.
/* FP:symbol.rs-2861 */ struct InternerInner {
/* FP:symbol.rs-2862 */     arena: DroplessArena,
/* FP:symbol.rs-2863 */     byte_strs: FxIndexSet<&'static [u8]>,
/* FP:symbol.rs-2864 */ }
/* FP:symbol.rs-2865 */ 
/* FP:symbol.rs-2866 */ impl Interner {
/* FP:symbol.rs-2867 */     // These arguments are `&str`, but because of the sharing, we are
/* FP:symbol.rs-2868 */     // effectively pre-interning all these strings for both `Symbol` and
/* FP:symbol.rs-2869 */     // `ByteSymbol`.
/* FP:symbol.rs-2870 */     fn prefill(init: &[&'static str], extra: &[&'static str]) -> Self {
/* FP:symbol.rs-2871 */         let byte_strs = FxIndexSet::from_iter(
/* FP:symbol.rs-2872 */             init.iter().copied().chain(extra.iter().copied()).map(|str| str.as_bytes()),
/* FP:symbol.rs-2873 */         );
/* FP:symbol.rs-2874 */ 
/* FP:symbol.rs-2875 */         // The order in which duplicates are reported is irrelevant.
/* FP:symbol.rs-2876 */         #[expect(rustc::potential_query_instability)]
/* FP:symbol.rs-2877 */         if byte_strs.len() != init.len() + extra.len() {
/* FP:symbol.rs-2878 */             panic!(
/* FP:symbol.rs-2879 */                 "duplicate symbols in the rustc symbol list and the extra symbols added by the driver: {:?}",
/* FP:symbol.rs-2880 */                 FxHashSet::intersection(
/* FP:symbol.rs-2881 */                     &init.iter().copied().collect(),
/* FP:symbol.rs-2882 */                     &extra.iter().copied().collect(),
/* FP:symbol.rs-2883 */                 )
/* FP:symbol.rs-2884 */                 .collect::<Vec<_>>()
/* FP:symbol.rs-2885 */             )
/* FP:symbol.rs-2886 */         }
/* FP:symbol.rs-2887 */ 
/* FP:symbol.rs-2888 */         Interner(Lock::new(InternerInner { arena: Default::default(), byte_strs }))
/* FP:symbol.rs-2889 */     }
/* FP:symbol.rs-2890 */ 
/* FP:symbol.rs-2891 */     fn intern_str(&self, str: &str) -> Symbol {
/* FP:symbol.rs-2892 */         Symbol::new(self.intern_inner(str.as_bytes()))
/* FP:symbol.rs-2893 */     }
/* FP:symbol.rs-2894 */ 
/* FP:symbol.rs-2895 */     fn intern_byte_str(&self, byte_str: &[u8]) -> ByteSymbol {
/* FP:symbol.rs-2896 */         ByteSymbol::new(self.intern_inner(byte_str))
/* FP:symbol.rs-2897 */     }
/* FP:symbol.rs-2898 */ 
/* FP:symbol.rs-2899 */     #[inline]
/* FP:symbol.rs-2900 */     fn intern_inner(&self, byte_str: &[u8]) -> u32 {
/* FP:symbol.rs-2901 */         let mut inner = self.0.lock();
/* FP:symbol.rs-2902 */         if let Some(idx) = inner.byte_strs.get_index_of(byte_str) {
/* FP:symbol.rs-2903 */             return idx as u32;
/* FP:symbol.rs-2904 */         }
/* FP:symbol.rs-2905 */ 
/* FP:symbol.rs-2906 */         let byte_str: &[u8] = inner.arena.alloc_slice(byte_str);
/* FP:symbol.rs-2907 */ 
/* FP:symbol.rs-2908 */         // SAFETY: we can extend the arena allocation to `'static` because we
/* FP:symbol.rs-2909 */         // only access these while the arena is still alive.
/* FP:symbol.rs-2910 */         let byte_str: &'static [u8] = unsafe { &*(byte_str as *const [u8]) };
/* FP:symbol.rs-2911 */ 
/* FP:symbol.rs-2912 */         // This second hash table lookup can be avoided by using `RawEntryMut`,
/* FP:symbol.rs-2913 */         // but this code path isn't hot enough for it to be worth it. See
/* FP:symbol.rs-2914 */         // #91445 for details.
/* FP:symbol.rs-2915 */         let (idx, is_new) = inner.byte_strs.insert_full(byte_str);
/* FP:symbol.rs-2916 */         debug_assert!(is_new); // due to the get_index_of check above
/* FP:symbol.rs-2917 */ 
/* FP:symbol.rs-2918 */         idx as u32
/* FP:symbol.rs-2919 */     }
/* FP:symbol.rs-2920 */ 
/* FP:symbol.rs-2921 */     /// Get the symbol as a string.
/* FP:symbol.rs-2922 */     ///
/* FP:symbol.rs-2923 */     /// [`Symbol::as_str()`] should be used in preference to this function.
/* FP:symbol.rs-2924 */     fn get_str(&self, symbol: Symbol) -> &str {
/* FP:symbol.rs-2925 */         let byte_str = self.get_inner(symbol.0.as_usize());
/* FP:symbol.rs-2926 */         // SAFETY: known to be a UTF8 string because it's a `Symbol`.
/* FP:symbol.rs-2927 */         unsafe { str::from_utf8_unchecked(byte_str) }
/* FP:symbol.rs-2928 */     }
/* FP:symbol.rs-2929 */ 
/* FP:symbol.rs-2930 */     /// Get the symbol as a string.
/* FP:symbol.rs-2931 */     ///
/* FP:symbol.rs-2932 */     /// [`ByteSymbol::as_byte_str()`] should be used in preference to this function.
/* FP:symbol.rs-2933 */     fn get_byte_str(&self, symbol: ByteSymbol) -> &[u8] {
/* FP:symbol.rs-2934 */         self.get_inner(symbol.0.as_usize())
/* FP:symbol.rs-2935 */     }
/* FP:symbol.rs-2936 */ 
/* FP:symbol.rs-2937 */     fn get_inner(&self, index: usize) -> &[u8] {
/* FP:symbol.rs-2938 */         self.0.lock().byte_strs.get_index(index).unwrap()
/* FP:symbol.rs-2939 */     }
/* FP:symbol.rs-2940 */ }
/* FP:symbol.rs-2941 */ 
/* FP:symbol.rs-2942 */ // This module has a very short name because it's used a lot.
/* FP:symbol.rs-2943 */ /// This module contains all the defined keyword `Symbol`s.
/* FP:symbol.rs-2944 */ ///
/* FP:symbol.rs-2945 */ /// Given that `kw` is imported, use them like `kw::keyword_name`.
/* FP:symbol.rs-2946 */ /// For example `kw::Loop` or `kw::Break`.
/* FP:symbol.rs-2947 */ pub mod kw {
/* FP:symbol.rs-2948 */     pub use super::kw_generated::*;
/* FP:symbol.rs-2949 */ }
/* FP:symbol.rs-2950 */ 
/* FP:symbol.rs-2951 */ // This module has a very short name because it's used a lot.
/* FP:symbol.rs-2952 */ /// This module contains all the defined non-keyword `Symbol`s.
/* FP:symbol.rs-2953 */ ///
/* FP:symbol.rs-2954 */ /// Given that `sym` is imported, use them like `sym::symbol_name`.
/* FP:symbol.rs-2955 */ /// For example `sym::rustfmt` or `sym::u8`.
/* FP:symbol.rs-2956 */ pub mod sym {
/* FP:symbol.rs-2957 */     // Used from a macro in `librustc_feature/accepted.rs`
/* FP:symbol.rs-2958 */     use super::Symbol;
/* FP:symbol.rs-2959 */     pub use super::kw::MacroRules as macro_rules;
/* FP:symbol.rs-2960 */     #[doc(inline)]
/* FP:symbol.rs-2961 */     pub use super::sym_generated::*;
/* FP:symbol.rs-2962 */ 
/* FP:symbol.rs-2963 */     /// Get the symbol for an integer.
/* FP:symbol.rs-2964 */     ///
/* FP:symbol.rs-2965 */     /// The first few non-negative integers each have a static symbol and therefore
/* FP:symbol.rs-2966 */     /// are fast.
/* FP:symbol.rs-2967 */     pub fn integer<N: TryInto<usize> + Copy + itoa::Integer>(n: N) -> Symbol {
/* FP:symbol.rs-2968 */         if let Result::Ok(idx) = n.try_into() {
/* FP:symbol.rs-2969 */             if idx < 10 {
/* FP:symbol.rs-2970 */                 return Symbol::new(super::SYMBOL_DIGITS_BASE + idx as u32);
/* FP:symbol.rs-2971 */             }
/* FP:symbol.rs-2972 */         }
/* FP:symbol.rs-2973 */         let mut buffer = itoa::Buffer::new();
/* FP:symbol.rs-2974 */         let printed = buffer.format(n);
/* FP:symbol.rs-2975 */         Symbol::intern(printed)
/* FP:symbol.rs-2976 */     }
/* FP:symbol.rs-2977 */ }
/* FP:symbol.rs-2978 */ 
/* FP:symbol.rs-2979 */ impl Symbol {
/* FP:symbol.rs-2980 */     fn is_special(self) -> bool {
/* FP:symbol.rs-2981 */         self <= kw::Underscore
/* FP:symbol.rs-2982 */     }
/* FP:symbol.rs-2983 */ 
/* FP:symbol.rs-2984 */     fn is_used_keyword_always(self) -> bool {
/* FP:symbol.rs-2985 */         self >= kw::As && self <= kw::While
/* FP:symbol.rs-2986 */     }
/* FP:symbol.rs-2987 */ 
/* FP:symbol.rs-2988 */     fn is_unused_keyword_always(self) -> bool {
/* FP:symbol.rs-2989 */         self >= kw::Abstract && self <= kw::Yield
/* FP:symbol.rs-2990 */     }
/* FP:symbol.rs-2991 */ 
/* FP:symbol.rs-2992 */     fn is_used_keyword_conditional(self, edition: impl FnOnce() -> Edition) -> bool {
/* FP:symbol.rs-2993 */         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
/* FP:symbol.rs-2994 */     }
/* FP:symbol.rs-2995 */ 
/* FP:symbol.rs-2996 */     fn is_unused_keyword_conditional(self, edition: impl Copy + FnOnce() -> Edition) -> bool {
/* FP:symbol.rs-2997 */         self == kw::Gen && edition().at_least_rust_2024()
/* FP:symbol.rs-2998 */             || self == kw::Try && edition().at_least_rust_2018()
/* FP:symbol.rs-2999 */     }
/* FP:symbol.rs-3000 */ 
/* FP:symbol.rs-3001 */     pub fn is_reserved(self, edition: impl Copy + FnOnce() -> Edition) -> bool {
/* FP:symbol.rs-3002 */         self.is_special()
/* FP:symbol.rs-3003 */             || self.is_used_keyword_always()
/* FP:symbol.rs-3004 */             || self.is_unused_keyword_always()
/* FP:symbol.rs-3005 */             || self.is_used_keyword_conditional(edition)
/* FP:symbol.rs-3006 */             || self.is_unused_keyword_conditional(edition)
/* FP:symbol.rs-3007 */     }
/* FP:symbol.rs-3008 */ 
/* FP:symbol.rs-3009 */     pub fn is_weak(self) -> bool {
/* FP:symbol.rs-3010 */         self >= kw::Auto && self <= kw::Yeet
/* FP:symbol.rs-3011 */     }
/* FP:symbol.rs-3012 */ 
/* FP:symbol.rs-3013 */     /// A keyword or reserved identifier that can be used as a path segment.
/* FP:symbol.rs-3014 */     pub fn is_path_segment_keyword(self) -> bool {
/* FP:symbol.rs-3015 */         self == kw::Super
/* FP:symbol.rs-3016 */             || self == kw::SelfLower
/* FP:symbol.rs-3017 */             || self == kw::SelfUpper
/* FP:symbol.rs-3018 */             || self == kw::Crate
/* FP:symbol.rs-3019 */             || self == kw::PathRoot
/* FP:symbol.rs-3020 */             || self == kw::DollarCrate
/* FP:symbol.rs-3021 */     }
/* FP:symbol.rs-3022 */ 
/* FP:symbol.rs-3023 */     /// Returns `true` if the symbol is `true` or `false`.
/* FP:symbol.rs-3024 */     pub fn is_bool_lit(self) -> bool {
/* FP:symbol.rs-3025 */         self == kw::True || self == kw::False
/* FP:symbol.rs-3026 */     }
/* FP:symbol.rs-3027 */ 
/* FP:symbol.rs-3028 */     /// Returns `true` if this symbol can be a raw identifier.
/* FP:symbol.rs-3029 */     pub fn can_be_raw(self) -> bool {
/* FP:symbol.rs-3030 */         self != sym::empty && self != kw::Underscore && !self.is_path_segment_keyword()
/* FP:symbol.rs-3031 */     }
/* FP:symbol.rs-3032 */ 
/* FP:symbol.rs-3033 */     /// Was this symbol index predefined in the compiler's `symbols!` macro?
/* FP:symbol.rs-3034 */     /// Note: this applies to both `Symbol`s and `ByteSymbol`s, which is why it
/* FP:symbol.rs-3035 */     /// takes a `u32` argument instead of a `&self` argument. Use with care.
/* FP:symbol.rs-3036 */     pub fn is_predefined(index: u32) -> bool {
/* FP:symbol.rs-3037 */         index < PREDEFINED_SYMBOLS_COUNT
/* FP:symbol.rs-3038 */     }
/* FP:symbol.rs-3039 */ }
/* FP:symbol.rs-3040 */ 
/* FP:symbol.rs-3041 */ impl Ident {
/* FP:symbol.rs-3042 */     /// Returns `true` for reserved identifiers used internally for elided lifetimes,
/* FP:symbol.rs-3043 */     /// unnamed method parameters, crate root module, error recovery etc.
/* FP:symbol.rs-3044 */     pub fn is_special(self) -> bool {
/* FP:symbol.rs-3045 */         self.name.is_special()
/* FP:symbol.rs-3046 */     }
/* FP:symbol.rs-3047 */ 
/* FP:symbol.rs-3048 */     /// Returns `true` if the token is a keyword used in the language.
/* FP:symbol.rs-3049 */     pub fn is_used_keyword(self) -> bool {
/* FP:symbol.rs-3050 */         // Note: `span.edition()` is relatively expensive, don't call it unless necessary.
/* FP:symbol.rs-3051 */         self.name.is_used_keyword_always()
/* FP:symbol.rs-3052 */             || self.name.is_used_keyword_conditional(|| self.span.edition())
/* FP:symbol.rs-3053 */     }
/* FP:symbol.rs-3054 */ 
/* FP:symbol.rs-3055 */     /// Returns `true` if the token is a keyword reserved for possible future use.
/* FP:symbol.rs-3056 */     pub fn is_unused_keyword(self) -> bool {
/* FP:symbol.rs-3057 */         // Note: `span.edition()` is relatively expensive, don't call it unless necessary.
/* FP:symbol.rs-3058 */         self.name.is_unused_keyword_always()
/* FP:symbol.rs-3059 */             || self.name.is_unused_keyword_conditional(|| self.span.edition())
/* FP:symbol.rs-3060 */     }
/* FP:symbol.rs-3061 */ 
/* FP:symbol.rs-3062 */     /// Returns `true` if the token is either a special identifier or a keyword.
/* FP:symbol.rs-3063 */     pub fn is_reserved(self) -> bool {
/* FP:symbol.rs-3064 */         // Note: `span.edition()` is relatively expensive, don't call it unless necessary.
/* FP:symbol.rs-3065 */         self.name.is_reserved(|| self.span.edition())
/* FP:symbol.rs-3066 */     }
/* FP:symbol.rs-3067 */ 
/* FP:symbol.rs-3068 */     /// A keyword or reserved identifier that can be used as a path segment.
/* FP:symbol.rs-3069 */     pub fn is_path_segment_keyword(self) -> bool {
/* FP:symbol.rs-3070 */         self.name.is_path_segment_keyword()
/* FP:symbol.rs-3071 */     }
/* FP:symbol.rs-3072 */ 
/* FP:symbol.rs-3073 */     /// We see this identifier in a normal identifier position, like variable name or a type.
/* FP:symbol.rs-3074 */     /// How was it written originally? Did it use the raw form? Let's try to guess.
/* FP:symbol.rs-3075 */     pub fn is_raw_guess(self) -> bool {
/* FP:symbol.rs-3076 */         self.name.can_be_raw() && self.is_reserved()
/* FP:symbol.rs-3077 */     }
/* FP:symbol.rs-3078 */ 
/* FP:symbol.rs-3079 */     /// Given the name of a lifetime without the first quote (`'`),
/* FP:symbol.rs-3080 */     /// returns whether the lifetime name is reserved (therefore invalid)
/* FP:symbol.rs-3081 */     pub fn is_reserved_lifetime(self) -> bool {
/* FP:symbol.rs-3082 */         self.is_reserved() && ![kw::Underscore, kw::Static].contains(&self.name)
/* FP:symbol.rs-3083 */     }
/* FP:symbol.rs-3084 */ 
/* FP:symbol.rs-3085 */     pub fn is_raw_lifetime_guess(self) -> bool {
/* FP:symbol.rs-3086 */         let name_without_apostrophe = self.without_first_quote();
/* FP:symbol.rs-3087 */         name_without_apostrophe.name != self.name
/* FP:symbol.rs-3088 */             && name_without_apostrophe.name.can_be_raw()
/* FP:symbol.rs-3089 */             && name_without_apostrophe.is_reserved_lifetime()
/* FP:symbol.rs-3090 */     }
/* FP:symbol.rs-3091 */ 
/* FP:symbol.rs-3092 */     pub fn guess_print_mode(self) -> IdentPrintMode {
/* FP:symbol.rs-3093 */         if self.is_raw_lifetime_guess() {
/* FP:symbol.rs-3094 */             IdentPrintMode::RawLifetime
/* FP:symbol.rs-3095 */         } else if self.is_raw_guess() {
/* FP:symbol.rs-3096 */             IdentPrintMode::RawIdent
/* FP:symbol.rs-3097 */         } else {
/* FP:symbol.rs-3098 */             IdentPrintMode::Normal
/* FP:symbol.rs-3099 */         }
/* FP:symbol.rs-3100 */     }
/* FP:symbol.rs-3101 */ 
/* FP:symbol.rs-3102 */     /// Whether this would be the identifier for a tuple field like `self.0`, as
/* FP:symbol.rs-3103 */     /// opposed to a named field like `self.thing`.
/* FP:symbol.rs-3104 */     pub fn is_numeric(self) -> bool {
/* FP:symbol.rs-3105 */         self.as_str().bytes().all(|b| b.is_ascii_digit())
/* FP:symbol.rs-3106 */     }
/* FP:symbol.rs-3107 */ }
/* FP:symbol.rs-3108 */ 
/* FP:symbol.rs-3109 */ /// Collect all the keywords in a given edition into a vector.
/* FP:symbol.rs-3110 */ ///
/* FP:symbol.rs-3111 */ /// *Note:* Please update this if a new keyword is added beyond the current
/* FP:symbol.rs-3112 */ /// range.
/* FP:symbol.rs-3113 */ pub fn used_keywords(edition: impl Copy + FnOnce() -> Edition) -> Vec<Symbol> {
/* FP:symbol.rs-3114 */     (kw::DollarCrate.as_u32()..kw::Yeet.as_u32())
/* FP:symbol.rs-3115 */         .filter_map(|kw| {
/* FP:symbol.rs-3116 */             let kw = Symbol::new(kw);
/* FP:symbol.rs-3117 */             if kw.is_used_keyword_always() || kw.is_used_keyword_conditional(edition) {
/* FP:symbol.rs-3118 */                 Some(kw)
/* FP:symbol.rs-3119 */             } else {
/* FP:symbol.rs-3120 */                 None
/* FP:symbol.rs-3121 */             }
/* FP:symbol.rs-3122 */         })
/* FP:symbol.rs-3123 */         .collect()
/* FP:symbol.rs-3124 */ }