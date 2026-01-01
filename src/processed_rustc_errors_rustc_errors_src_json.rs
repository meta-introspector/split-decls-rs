/* FP:json.rs-0001 */ // A JSON emitter for errors.
/* FP:json.rs-0002 */ //
/* FP:json.rs-0003 */ // This works by converting errors to a simplified structural format (see the
/* FP:json.rs-0004 */ // structs at the start of the file) and then serializing them. These should
/* FP:json.rs-0005 */ // contain as much information about the error as possible.
/* FP:json.rs-0006 */ //
/* FP:json.rs-0007 */ // The format of the JSON output should be considered *unstable*. For now the
/* FP:json.rs-0008 */ // structs at the end of this file (Diagnostic*) specify the error format.
/* FP:json.rs-0009 */ 
/* FP:json.rs-0010 */ // FIXME: spec the JSON output properly.
/* FP:json.rs-0011 */ 
/* FP:json.rs-0012 */ use std::error::Report;
/* FP:json.rs-0013 */ use std::io::{self, Write};
/* FP:json.rs-0014 */ use std::path::Path;
/* FP:json.rs-0015 */ use std::sync::{Arc, Mutex};
/* FP:json.rs-0016 */ use std::vec;
/* FP:json.rs-0017 */ 
/* FP:json.rs-0018 */ use derive_setters::Setters;
/* FP:json.rs-0019 */ use crate::rustc_data_structures::sync::IntoDynSyncSend;
/* FP:json.rs-0020 */ use crate::rustc_error_messages::FluentArgs;
/* FP:json.rs-0021 */ use crate::rustc_lint_defs::Applicability;
/* FP:json.rs-0022 */ use crate::rustc_complete::Span;
/* FP:json.rs-0023 */ use crate::rustc_complete::hygiene::ExpnData;
/* FP:json.rs-0024 */ use crate::rustc_complete::source_map::{FilePathMapping, SourceMap};
/* FP:json.rs-0025 */ use serde::Serialize;
/* FP:json.rs-0026 */ use termcolor::{ColorSpec, WriteColor};
/* FP:json.rs-0027 */ 
/* FP:json.rs-0028 */ use crate::diagnostic::IsLint;
/* FP:json.rs-0029 */ use crate::emitter::{
/* FP:json.rs-0030 */     ColorConfig, Destination, Emitter, HumanEmitter, HumanReadableErrorType, OutputTheme,
/* FP:json.rs-0031 */     TimingEvent, should_show_source_code,
/* FP:json.rs-0032 */ };
/* FP:json.rs-0033 */ use crate::registry::Registry;
/* FP:json.rs-0034 */ use crate::timings::{TimingRecord, TimingSection};
/* FP:json.rs-0035 */ use crate::translation::{Translator, to_fluent_args};
/* FP:json.rs-0036 */ use crate::{CodeSuggestion, MultiSpan, SpanLabel, Subdiag, Suggestions, TerminalUrl};
/* FP:json.rs-0037 */ 
/* FP:json.rs-0038 */ #[cfg(test)]
/* FP:json.rs-0040 */ 
/* FP:json.rs-0041 */ #[derive(Setters)]
/* FP:json.rs-0042 */ pub struct JsonEmitter {
/* FP:json.rs-0043 */     #[setters(skip)]
/* FP:json.rs-0044 */     dst: IntoDynSyncSend<Box<dyn Write + Send>>,
/* FP:json.rs-0045 */     #[setters(skip)]
/* FP:json.rs-0046 */     sm: Option<Arc<SourceMap>>,
/* FP:json.rs-0047 */     #[setters(skip)]
/* FP:json.rs-0048 */     translator: Translator,
/* FP:json.rs-0049 */     #[setters(skip)]
/* FP:json.rs-0050 */     pretty: bool,
/* FP:json.rs-0051 */     ui_testing: bool,
/* FP:json.rs-0052 */     ignored_directories_in_source_blocks: Vec<String>,
/* FP:json.rs-0053 */     #[setters(skip)]
/* FP:json.rs-0054 */     json_rendered: HumanReadableErrorType,
/* FP:json.rs-0055 */     color_config: ColorConfig,
/* FP:json.rs-0056 */     diagnostic_width: Option<usize>,
/* FP:json.rs-0057 */     macro_backtrace: bool,
/* FP:json.rs-0058 */     track_diagnostics: bool,
/* FP:json.rs-0059 */     terminal_url: TerminalUrl,
/* FP:json.rs-0060 */ }
/* FP:json.rs-0061 */ 
/* FP:json.rs-0062 */ impl JsonEmitter {
/* FP:json.rs-0063 */     pub fn new(
/* FP:json.rs-0064 */         dst: Box<dyn Write + Send>,
/* FP:json.rs-0065 */         sm: Option<Arc<SourceMap>>,
/* FP:json.rs-0066 */         translator: Translator,
/* FP:json.rs-0067 */         pretty: bool,
/* FP:json.rs-0068 */         json_rendered: HumanReadableErrorType,
/* FP:json.rs-0069 */         color_config: ColorConfig,
/* FP:json.rs-0070 */     ) -> JsonEmitter {
/* FP:json.rs-0071 */         JsonEmitter {
/* FP:json.rs-0072 */             dst: IntoDynSyncSend(dst),
/* FP:json.rs-0073 */             sm,
/* FP:json.rs-0074 */             translator,
/* FP:json.rs-0075 */             pretty,
/* FP:json.rs-0076 */             ui_testing: false,
/* FP:json.rs-0077 */             ignored_directories_in_source_blocks: Vec::new(),
/* FP:json.rs-0078 */             json_rendered,
/* FP:json.rs-0079 */             color_config,
/* FP:json.rs-0080 */             diagnostic_width: None,
/* FP:json.rs-0081 */             macro_backtrace: false,
/* FP:json.rs-0082 */             track_diagnostics: false,
/* FP:json.rs-0083 */             terminal_url: TerminalUrl::No,
/* FP:json.rs-0084 */         }
/* FP:json.rs-0085 */     }
/* FP:json.rs-0086 */ 
/* FP:json.rs-0087 */     fn emit(&mut self, val: EmitTyped<'_>) -> io::Result<()> {
/* FP:json.rs-0088 */         if self.pretty {
/* FP:json.rs-0089 */             serde_json::to_writer_pretty(&mut *self.dst, &val)?
/* FP:json.rs-0090 */         } else {
/* FP:json.rs-0091 */             serde_json::to_writer(&mut *self.dst, &val)?
/* FP:json.rs-0092 */         };
/* FP:json.rs-0093 */         self.dst.write_all(b"\n")?;
/* FP:json.rs-0094 */         self.dst.flush()
/* FP:json.rs-0095 */     }
/* FP:json.rs-0096 */ }
/* FP:json.rs-0097 */ 
/* FP:json.rs-0098 */ #[derive(Serialize)]
/* FP:json.rs-0099 */ #[serde(tag = "$message_type", rename_all = "snake_case")]
/* FP:json.rs-0100 */ enum EmitTyped<'a> {
/* FP:json.rs-0101 */     Diagnostic(Diagnostic),
/* FP:json.rs-0102 */     Artifact(ArtifactNotification<'a>),
/* FP:json.rs-0103 */     SectionTiming(SectionTimestamp<'a>),
/* FP:json.rs-0104 */     FutureIncompat(FutureIncompatReport<'a>),
/* FP:json.rs-0105 */     UnusedExtern(UnusedExterns<'a>),
/* FP:json.rs-0106 */ }
/* FP:json.rs-0107 */ 
/* FP:json.rs-0108 */ impl Emitter for JsonEmitter {
/* FP:json.rs-0109 */     fn emit_diagnostic(&mut self, diag: crate::DiagInner, registry: &Registry) {
/* FP:json.rs-0110 */         let data = Diagnostic::from_errors_diagnostic(diag, self, registry);
/* FP:json.rs-0111 */         let result = self.emit(EmitTyped::Diagnostic(data));
/* FP:json.rs-0112 */         if let Err(e) = result {
/* FP:json.rs-0113 */             panic!("failed to print diagnostics: {e:?}");
/* FP:json.rs-0114 */         }
/* FP:json.rs-0115 */     }
/* FP:json.rs-0116 */ 
/* FP:json.rs-0117 */     fn emit_artifact_notification(&mut self, path: &Path, artifact_type: &str) {
/* FP:json.rs-0118 */         let data = ArtifactNotification { artifact: path, emit: artifact_type };
/* FP:json.rs-0119 */         let result = self.emit(EmitTyped::Artifact(data));
/* FP:json.rs-0120 */         if let Err(e) = result {
/* FP:json.rs-0121 */             panic!("failed to print notification: {e:?}");
/* FP:json.rs-0122 */         }
/* FP:json.rs-0123 */     }
/* FP:json.rs-0124 */ 
/* FP:json.rs-0125 */     fn emit_timing_section(&mut self, record: TimingRecord, event: TimingEvent) {
/* FP:json.rs-0126 */         let event = match event {
/* FP:json.rs-0127 */             TimingEvent::Start => "start",
/* FP:json.rs-0128 */             TimingEvent::End => "end",
/* FP:json.rs-0129 */         };
/* FP:json.rs-0130 */         let name = match record.section {
/* FP:json.rs-0131 */             TimingSection::Linking => "link",
/* FP:json.rs-0132 */             TimingSection::Codegen => "codegen",
/* FP:json.rs-0133 */         };
/* FP:json.rs-0134 */         let data = SectionTimestamp { name, event, timestamp: record.timestamp };
/* FP:json.rs-0135 */         let result = self.emit(EmitTyped::SectionTiming(data));
/* FP:json.rs-0136 */         if let Err(e) = result {
/* FP:json.rs-0137 */             panic!("failed to print timing section: {e:?}");
/* FP:json.rs-0138 */         }
/* FP:json.rs-0139 */     }
/* FP:json.rs-0140 */ 
/* FP:json.rs-0141 */     fn emit_future_breakage_report(&mut self, diags: Vec<crate::DiagInner>, registry: &Registry) {
/* FP:json.rs-0142 */         let data: Vec<FutureBreakageItem<'_>> = diags
/* FP:json.rs-0143 */             .into_iter()
/* FP:json.rs-0144 */             .map(|mut diag| {
/* FP:json.rs-0145 */                 // Allowed or expected lints don't normally (by definition) emit a lint
/* FP:json.rs-0146 */                 // but future incompat lints are special and are emitted anyway.
/* FP:json.rs-0147 */                 //
/* FP:json.rs-0148 */                 // So to avoid ICEs and confused users we "upgrade" the lint level for
/* FP:json.rs-0149 */                 // those `FutureBreakageItem` to warn.
/* FP:json.rs-0150 */                 if matches!(diag.level, crate::Level::Allow | crate::Level::Expect) {
/* FP:json.rs-0151 */                     diag.level = crate::Level::Warning;
/* FP:json.rs-0152 */                 }
/* FP:json.rs-0153 */                 FutureBreakageItem {
/* FP:json.rs-0154 */                     diagnostic: EmitTyped::Diagnostic(Diagnostic::from_errors_diagnostic(
/* FP:json.rs-0155 */                         diag, self, registry,
/* FP:json.rs-0156 */                     )),
/* FP:json.rs-0157 */                 }
/* FP:json.rs-0158 */             })
/* FP:json.rs-0159 */             .collect();
/* FP:json.rs-0160 */         let report = FutureIncompatReport { future_incompat_report: data };
/* FP:json.rs-0161 */         let result = self.emit(EmitTyped::FutureIncompat(report));
/* FP:json.rs-0162 */         if let Err(e) = result {
/* FP:json.rs-0163 */             panic!("failed to print future breakage report: {e:?}");
/* FP:json.rs-0164 */         }
/* FP:json.rs-0165 */     }
/* FP:json.rs-0166 */ 
/* FP:json.rs-0167 */     fn emit_unused_externs(&mut self, lint_level: crate::rustc_lint_defs::Level, unused_externs: &[&str]) {
/* FP:json.rs-0168 */         let lint_level = lint_level.as_str();
/* FP:json.rs-0169 */         let data = UnusedExterns { lint_level, unused_extern_names: unused_externs };
/* FP:json.rs-0170 */         let result = self.emit(EmitTyped::UnusedExtern(data));
/* FP:json.rs-0171 */         if let Err(e) = result {
/* FP:json.rs-0172 */             panic!("failed to print unused externs: {e:?}");
/* FP:json.rs-0173 */         }
/* FP:json.rs-0174 */     }
/* FP:json.rs-0175 */ 
/* FP:json.rs-0176 */     fn source_map(&self) -> Option<&SourceMap> {
/* FP:json.rs-0177 */         self.sm.as_deref()
/* FP:json.rs-0178 */     }
/* FP:json.rs-0179 */ 
/* FP:json.rs-0180 */     fn should_show_explain(&self) -> bool {
/* FP:json.rs-0181 */         !self.json_rendered.short()
/* FP:json.rs-0182 */     }
/* FP:json.rs-0183 */ 
/* FP:json.rs-0184 */     fn translator(&self) -> &Translator {
/* FP:json.rs-0185 */         &self.translator
/* FP:json.rs-0186 */     }
/* FP:json.rs-0187 */ }
/* FP:json.rs-0188 */ 
/* FP:json.rs-0189 */ // The following data types are provided just for serialisation.
/* FP:json.rs-0190 */ 
/* FP:json.rs-0191 */ #[derive(Serialize)]
/* FP:json.rs-0192 */ struct Diagnostic {
/* FP:json.rs-0193 */     /// The primary error message.
/* FP:json.rs-0194 */     message: String,
/* FP:json.rs-0195 */     code: Option<DiagnosticCode>,
/* FP:json.rs-0196 */     /// "error: internal compiler error", "error", "warning", "note", "help".
/* FP:json.rs-0197 */     level: &'static str,
/* FP:json.rs-0198 */     spans: Vec<DiagnosticSpan>,
/* FP:json.rs-0199 */     /// Associated diagnostic messages.
/* FP:json.rs-0200 */     children: Vec<Diagnostic>,
/* FP:json.rs-0201 */     /// The message as rustc would render it.
/* FP:json.rs-0202 */     rendered: Option<String>,
/* FP:json.rs-0203 */ }
/* FP:json.rs-0204 */ 
/* FP:json.rs-0205 */ #[derive(Serialize)]
/* FP:json.rs-0206 */ struct DiagnosticSpan {
/* FP:json.rs-0207 */     file_name: String,
/* FP:json.rs-0208 */     byte_start: u32,
/* FP:json.rs-0209 */     byte_end: u32,
/* FP:json.rs-0210 */     /// 1-based.
/* FP:json.rs-0211 */     line_start: usize,
/* FP:json.rs-0212 */     line_end: usize,
/* FP:json.rs-0213 */     /// 1-based, character offset.
/* FP:json.rs-0214 */     column_start: usize,
/* FP:json.rs-0215 */     column_end: usize,
/* FP:json.rs-0216 */     /// Is this a "primary" span -- meaning the point, or one of the points,
/* FP:json.rs-0217 */     /// where the error occurred?
/* FP:json.rs-0218 */     is_primary: bool,
/* FP:json.rs-0219 */     /// Source text from the start of line_start to the end of line_end.
/* FP:json.rs-0220 */     text: Vec<DiagnosticSpanLine>,
/* FP:json.rs-0221 */     /// Label that should be placed at this location (if any)
/* FP:json.rs-0222 */     label: Option<String>,
/* FP:json.rs-0223 */     /// If we are suggesting a replacement, this will contain text
/* FP:json.rs-0224 */     /// that should be sliced in atop this span.
/* FP:json.rs-0225 */     suggested_replacement: Option<String>,
/* FP:json.rs-0226 */     /// If the suggestion is approximate
/* FP:json.rs-0227 */     suggestion_applicability: Option<Applicability>,
/* FP:json.rs-0228 */     /// Macro invocations that created the code at this span, if any.
/* FP:json.rs-0229 */     expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
/* FP:json.rs-0230 */ }
/* FP:json.rs-0231 */ 
/* FP:json.rs-0232 */ #[derive(Serialize)]
/* FP:json.rs-0233 */ struct DiagnosticSpanLine {
/* FP:json.rs-0234 */     text: String,
/* FP:json.rs-0235 */ 
/* FP:json.rs-0236 */     /// 1-based, character offset in self.text.
/* FP:json.rs-0237 */     highlight_start: usize,
/* FP:json.rs-0238 */ 
/* FP:json.rs-0239 */     highlight_end: usize,
/* FP:json.rs-0240 */ }
/* FP:json.rs-0241 */ 
/* FP:json.rs-0242 */ #[derive(Serialize)]
/* FP:json.rs-0243 */ struct DiagnosticSpanMacroExpansion {
/* FP:json.rs-0244 */     /// span where macro was applied to generate this code; note that
/* FP:json.rs-0245 */     /// this may itself derive from a macro (if
/* FP:json.rs-0246 */     /// `span.expansion.is_some()`)
/* FP:json.rs-0247 */     span: DiagnosticSpan,
/* FP:json.rs-0248 */ 
/* FP:json.rs-0249 */     /// name of macro that was applied (e.g., "foo!" or "#[derive(Eq)]")
/* FP:json.rs-0250 */     macro_decl_name: String,
/* FP:json.rs-0251 */ 
/* FP:json.rs-0252 */     /// span where macro was defined (if known)
/* FP:json.rs-0253 */     def_site_span: DiagnosticSpan,
/* FP:json.rs-0254 */ }
/* FP:json.rs-0255 */ 
/* FP:json.rs-0256 */ #[derive(Serialize)]
/* FP:json.rs-0257 */ struct DiagnosticCode {
/* FP:json.rs-0258 */     /// The error code (e.g. "E1234"), if the diagnostic has one. Or the lint
/* FP:json.rs-0259 */     /// name, if it's a lint without an error code.
/* FP:json.rs-0260 */     code: String,
/* FP:json.rs-0261 */     /// An explanation for the code.
/* FP:json.rs-0262 */     explanation: Option<&'static str>,
/* FP:json.rs-0263 */ }
/* FP:json.rs-0264 */ 
/* FP:json.rs-0265 */ #[derive(Serialize)]
/* FP:json.rs-0266 */ struct ArtifactNotification<'a> {
/* FP:json.rs-0267 */     /// The path of the artifact.
/* FP:json.rs-0268 */     artifact: &'a Path,
/* FP:json.rs-0269 */     /// What kind of artifact we're emitting.
/* FP:json.rs-0270 */     emit: &'a str,
/* FP:json.rs-0271 */ }
/* FP:json.rs-0272 */ 
/* FP:json.rs-0273 */ #[derive(Serialize)]
/* FP:json.rs-0274 */ struct SectionTimestamp<'a> {
/* FP:json.rs-0275 */     /// Name of the section
/* FP:json.rs-0276 */     name: &'a str,
/* FP:json.rs-0277 */     /// Start/end of the section
/* FP:json.rs-0278 */     event: &'a str,
/* FP:json.rs-0279 */     /// Opaque timestamp.
/* FP:json.rs-0280 */     timestamp: u128,
/* FP:json.rs-0281 */ }
/* FP:json.rs-0282 */ 
/* FP:json.rs-0283 */ #[derive(Serialize)]
/* FP:json.rs-0284 */ struct FutureBreakageItem<'a> {
/* FP:json.rs-0285 */     // Always EmitTyped::Diagnostic, but we want to make sure it gets serialized
/* FP:json.rs-0286 */     // with "$message_type".
/* FP:json.rs-0287 */     diagnostic: EmitTyped<'a>,
/* FP:json.rs-0288 */ }
/* FP:json.rs-0289 */ 
/* FP:json.rs-0290 */ #[derive(Serialize)]
/* FP:json.rs-0291 */ struct FutureIncompatReport<'a> {
/* FP:json.rs-0292 */     future_incompat_report: Vec<FutureBreakageItem<'a>>,
/* FP:json.rs-0293 */ }
/* FP:json.rs-0294 */ 
/* FP:json.rs-0295 */ // NOTE: Keep this in sync with the equivalent structs in rustdoc's
/* FP:json.rs-0296 */ // doctest component (as well as cargo).
/* FP:json.rs-0297 */ // We could unify this struct the one in rustdoc but they have different
/* FP:json.rs-0298 */ // ownership semantics, so doing so would create wasteful allocations.
/* FP:json.rs-0299 */ #[derive(Serialize)]
/* FP:json.rs-0300 */ struct UnusedExterns<'a> {
/* FP:json.rs-0301 */     /// The severity level of the unused dependencies lint
/* FP:json.rs-0302 */     lint_level: &'a str,
/* FP:json.rs-0303 */     /// List of unused externs by their names.
/* FP:json.rs-0304 */     unused_extern_names: &'a [&'a str],
/* FP:json.rs-0305 */ }
/* FP:json.rs-0306 */ 
/* FP:json.rs-0307 */ impl Diagnostic {
/* FP:json.rs-0308 */     /// Converts from `crate::rustc_errors::DiagInner` to `Diagnostic`.
/* FP:json.rs-0309 */     fn from_errors_diagnostic(
/* FP:json.rs-0310 */         diag: crate::DiagInner,
/* FP:json.rs-0311 */         je: &JsonEmitter,
/* FP:json.rs-0312 */         registry: &Registry,
/* FP:json.rs-0313 */     ) -> Diagnostic {
/* FP:json.rs-0314 */         let args = to_fluent_args(diag.args.iter());
/* FP:json.rs-0315 */         let sugg_to_diag = |sugg: &CodeSuggestion| {
/* FP:json.rs-0316 */             let translated_message =
/* FP:json.rs-0317 */                 je.translator.translate_message(&sugg.msg, &args).map_err(Report::new).unwrap();
/* FP:json.rs-0318 */             Diagnostic {
/* FP:json.rs-0319 */                 message: translated_message.to_string(),
/* FP:json.rs-0320 */                 code: None,
/* FP:json.rs-0321 */                 level: "help",
/* FP:json.rs-0322 */                 spans: DiagnosticSpan::from_suggestion(sugg, &args, je),
/* FP:json.rs-0323 */                 children: vec![],
/* FP:json.rs-0324 */                 rendered: None,
/* FP:json.rs-0325 */             }
/* FP:json.rs-0326 */         };
/* FP:json.rs-0327 */         let sugg = match &diag.suggestions {
/* FP:json.rs-0328 */             Suggestions::Enabled(suggestions) => suggestions.iter().map(sugg_to_diag),
/* FP:json.rs-0329 */             Suggestions::Sealed(suggestions) => suggestions.iter().map(sugg_to_diag),
/* FP:json.rs-0330 */             Suggestions::Disabled => [].iter().map(sugg_to_diag),
/* FP:json.rs-0331 */         };
/* FP:json.rs-0332 */ 
/* FP:json.rs-0333 */         // generate regular command line output and store it in the json
/* FP:json.rs-0334 */ 
/* FP:json.rs-0335 */         // A threadsafe buffer for writing.
/* FP:json.rs-0336 */         #[derive(Default, Clone)]
/* FP:json.rs-0337 */         struct BufWriter(Arc<Mutex<Vec<u8>>>);
/* FP:json.rs-0338 */ 
/* FP:json.rs-0339 */         impl Write for BufWriter {
/* FP:json.rs-0340 */             fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
/* FP:json.rs-0341 */                 self.0.lock().unwrap().write(buf)
/* FP:json.rs-0342 */             }
/* FP:json.rs-0343 */             fn flush(&mut self) -> io::Result<()> {
/* FP:json.rs-0344 */                 self.0.lock().unwrap().flush()
/* FP:json.rs-0345 */             }
/* FP:json.rs-0346 */         }
/* FP:json.rs-0347 */         impl WriteColor for BufWriter {
/* FP:json.rs-0348 */             fn supports_color(&self) -> bool {
/* FP:json.rs-0349 */                 false
/* FP:json.rs-0350 */             }
/* FP:json.rs-0351 */ 
/* FP:json.rs-0352 */             fn set_color(&mut self, _spec: &ColorSpec) -> io::Result<()> {
/* FP:json.rs-0353 */                 Ok(())
/* FP:json.rs-0354 */             }
/* FP:json.rs-0355 */ 
/* FP:json.rs-0356 */             fn reset(&mut self) -> io::Result<()> {
/* FP:json.rs-0357 */                 Ok(())
/* FP:json.rs-0358 */             }
/* FP:json.rs-0359 */         }
/* FP:json.rs-0360 */ 
/* FP:json.rs-0361 */         let translated_message = je.translator.translate_messages(&diag.messages, &args);
/* FP:json.rs-0362 */ 
/* FP:json.rs-0363 */         let code = if let Some(code) = diag.code {
/* FP:json.rs-0364 */             Some(DiagnosticCode {
/* FP:json.rs-0365 */                 code: code.to_string(),
/* FP:json.rs-0366 */                 explanation: registry.try_find_description(code).ok(),
/* FP:json.rs-0367 */             })
/* FP:json.rs-0368 */         } else if let Some(IsLint { name, .. }) = &diag.is_lint {
/* FP:json.rs-0369 */             Some(DiagnosticCode { code: name.to_string(), explanation: None })
/* FP:json.rs-0370 */         } else {
/* FP:json.rs-0371 */             None
/* FP:json.rs-0372 */         };
/* FP:json.rs-0373 */         let level = diag.level.to_str();
/* FP:json.rs-0374 */         let spans = DiagnosticSpan::from_multispan(&diag.span, &args, je);
/* FP:json.rs-0375 */         let mut children: Vec<Diagnostic> = diag
/* FP:json.rs-0376 */             .children
/* FP:json.rs-0377 */             .iter()
/* FP:json.rs-0378 */             .map(|c| Diagnostic::from_sub_diagnostic(c, &args, je))
/* FP:json.rs-0379 */             .chain(sugg)
/* FP:json.rs-0380 */             .collect();
/* FP:json.rs-0381 */         if je.track_diagnostics && diag.span.has_primary_spans() && !diag.span.is_dummy() {
/* FP:json.rs-0382 */             children
/* FP:json.rs-0383 */                 .insert(0, Diagnostic::from_sub_diagnostic(&diag.emitted_at_sub_diag(), &args, je));
/* FP:json.rs-0384 */         }
/* FP:json.rs-0385 */         let buf = BufWriter::default();
/* FP:json.rs-0386 */         let mut dst: Destination = Box::new(buf.clone());
/* FP:json.rs-0387 */         let short = je.json_rendered.short();
/* FP:json.rs-0388 */         match je.color_config {
/* FP:json.rs-0389 */             ColorConfig::Always | ColorConfig::Auto => dst = Box::new(termcolor::Ansi::new(dst)),
/* FP:json.rs-0390 */             ColorConfig::Never => {}
/* FP:json.rs-0391 */         }
/* FP:json.rs-0392 */         HumanEmitter::new(dst, je.translator.clone())
/* FP:json.rs-0393 */             .short_message(short)
/* FP:json.rs-0394 */             .sm(je.sm.clone())
/* FP:json.rs-0395 */             .diagnostic_width(je.diagnostic_width)
/* FP:json.rs-0396 */             .macro_backtrace(je.macro_backtrace)
/* FP:json.rs-0397 */             .track_diagnostics(je.track_diagnostics)
/* FP:json.rs-0398 */             .terminal_url(je.terminal_url)
/* FP:json.rs-0399 */             .ui_testing(je.ui_testing)
/* FP:json.rs-0400 */             .ignored_directories_in_source_blocks(je.ignored_directories_in_source_blocks.clone())
/* FP:json.rs-0401 */             .theme(if let HumanReadableErrorType::Unicode = je.json_rendered {
/* FP:json.rs-0402 */                 OutputTheme::Unicode
/* FP:json.rs-0403 */             } else {
/* FP:json.rs-0404 */                 OutputTheme::Ascii
/* FP:json.rs-0405 */             })
/* FP:json.rs-0406 */             .emit_diagnostic(diag, registry);
/* FP:json.rs-0407 */         let buf = Arc::try_unwrap(buf.0).unwrap().into_inner().unwrap();
/* FP:json.rs-0408 */         let buf = String::from_utf8(buf).unwrap();
/* FP:json.rs-0409 */ 
/* FP:json.rs-0410 */         Diagnostic {
/* FP:json.rs-0411 */             message: translated_message.to_string(),
/* FP:json.rs-0412 */             code,
/* FP:json.rs-0413 */             level,
/* FP:json.rs-0414 */             spans,
/* FP:json.rs-0415 */             children,
/* FP:json.rs-0416 */             rendered: Some(buf),
/* FP:json.rs-0417 */         }
/* FP:json.rs-0418 */     }
/* FP:json.rs-0419 */ 
/* FP:json.rs-0420 */     fn from_sub_diagnostic(
/* FP:json.rs-0421 */         subdiag: &Subdiag,
/* FP:json.rs-0422 */         args: &FluentArgs<'_>,
/* FP:json.rs-0423 */         je: &JsonEmitter,
/* FP:json.rs-0424 */     ) -> Diagnostic {
/* FP:json.rs-0425 */         let translated_message = je.translator.translate_messages(&subdiag.messages, args);
/* FP:json.rs-0426 */         Diagnostic {
/* FP:json.rs-0427 */             message: translated_message.to_string(),
/* FP:json.rs-0428 */             code: None,
/* FP:json.rs-0429 */             level: subdiag.level.to_str(),
/* FP:json.rs-0430 */             spans: DiagnosticSpan::from_multispan(&subdiag.span, args, je),
/* FP:json.rs-0431 */             children: vec![],
/* FP:json.rs-0432 */             rendered: None,
/* FP:json.rs-0433 */         }
/* FP:json.rs-0434 */     }
/* FP:json.rs-0435 */ }
/* FP:json.rs-0436 */ 
/* FP:json.rs-0437 */ impl DiagnosticSpan {
/* FP:json.rs-0438 */     fn from_span_label(
/* FP:json.rs-0439 */         span: SpanLabel,
/* FP:json.rs-0440 */         suggestion: Option<(&String, Applicability)>,
/* FP:json.rs-0441 */         args: &FluentArgs<'_>,
/* FP:json.rs-0442 */         je: &JsonEmitter,
/* FP:json.rs-0443 */     ) -> DiagnosticSpan {
/* FP:json.rs-0444 */         Self::from_span_etc(
/* FP:json.rs-0445 */             span.span,
/* FP:json.rs-0446 */             span.is_primary,
/* FP:json.rs-0447 */             span.label
/* FP:json.rs-0448 */                 .as_ref()
/* FP:json.rs-0449 */                 .map(|m| je.translator.translate_message(m, args).unwrap())
/* FP:json.rs-0450 */                 .map(|m| m.to_string()),
/* FP:json.rs-0451 */             suggestion,
/* FP:json.rs-0452 */             je,
/* FP:json.rs-0453 */         )
/* FP:json.rs-0454 */     }
/* FP:json.rs-0455 */ 
/* FP:json.rs-0456 */     fn from_span_etc(
/* FP:json.rs-0457 */         span: Span,
/* FP:json.rs-0458 */         is_primary: bool,
/* FP:json.rs-0459 */         label: Option<String>,
/* FP:json.rs-0460 */         suggestion: Option<(&String, Applicability)>,
/* FP:json.rs-0461 */         je: &JsonEmitter,
/* FP:json.rs-0462 */     ) -> DiagnosticSpan {
/* FP:json.rs-0463 */         // obtain the full backtrace from the `macro_backtrace`
/* FP:json.rs-0464 */         // helper; in some ways, it'd be better to expand the
/* FP:json.rs-0465 */         // backtrace ourselves, but the `macro_backtrace` helper makes
/* FP:json.rs-0466 */         // some decision, such as dropping some frames, and I don't
/* FP:json.rs-0467 */         // want to duplicate that logic here.
/* FP:json.rs-0468 */         let backtrace = span.macro_backtrace();
/* FP:json.rs-0469 */         DiagnosticSpan::from_span_full(span, is_primary, label, suggestion, backtrace, je)
/* FP:json.rs-0470 */     }
/* FP:json.rs-0471 */ 
/* FP:json.rs-0472 */     fn from_span_full(
/* FP:json.rs-0473 */         mut span: Span,
/* FP:json.rs-0474 */         is_primary: bool,
/* FP:json.rs-0475 */         label: Option<String>,
/* FP:json.rs-0476 */         suggestion: Option<(&String, Applicability)>,
/* FP:json.rs-0477 */         mut backtrace: impl Iterator<Item = ExpnData>,
/* FP:json.rs-0478 */         je: &JsonEmitter,
/* FP:json.rs-0479 */     ) -> DiagnosticSpan {
/* FP:json.rs-0480 */         let empty_source_map;
/* FP:json.rs-0481 */         let sm = match &je.sm {
/* FP:json.rs-0482 */             Some(s) => s,
/* FP:json.rs-0483 */             None => {
/* FP:json.rs-0484 */                 span = crate::rustc_span::DUMMY_SP;
/* FP:json.rs-0485 */                 empty_source_map = Arc::new(SourceMap::new(FilePathMapping::empty()));
/* FP:json.rs-0486 */                 empty_source_map
/* FP:json.rs-0487 */                     .new_source_file(std::path::PathBuf::from("empty.rs").into(), String::new());
/* FP:json.rs-0488 */                 &empty_source_map
/* FP:json.rs-0489 */             }
/* FP:json.rs-0490 */         };
/* FP:json.rs-0491 */         let start = sm.lookup_char_pos(span.lo());
/* FP:json.rs-0492 */         // If this goes from the start of a line to the end and the replacement
/* FP:json.rs-0493 */         // is an empty string, increase the length to include the newline so we don't
/* FP:json.rs-0494 */         // leave an empty line
/* FP:json.rs-0495 */         if start.col.0 == 0
/* FP:json.rs-0496 */             && let Some((suggestion, _)) = suggestion
/* FP:json.rs-0497 */             && suggestion.is_empty()
/* FP:json.rs-0498 */             && let Ok(after) = sm.span_to_next_source(span)
/* FP:json.rs-0499 */             && after.starts_with('\n')
/* FP:json.rs-0500 */         {
/* FP:json.rs-0501 */             span = span.with_hi(span.hi() + crate::rustc_span::BytePos(1));
/* FP:json.rs-0502 */         }
/* FP:json.rs-0503 */         let end = sm.lookup_char_pos(span.hi());
/* FP:json.rs-0504 */         let backtrace_step = backtrace.next().map(|bt| {
/* FP:json.rs-0505 */             let call_site = Self::from_span_full(bt.call_site, false, None, None, backtrace, je);
/* FP:json.rs-0506 */             let def_site_span = Self::from_span_full(
/* FP:json.rs-0507 */                 sm.guess_head_span(bt.def_site),
/* FP:json.rs-0508 */                 false,
/* FP:json.rs-0509 */                 None,
/* FP:json.rs-0510 */                 None,
/* FP:json.rs-0511 */                 [].into_iter(),
/* FP:json.rs-0512 */                 je,
/* FP:json.rs-0513 */             );
/* FP:json.rs-0514 */             Box::new(DiagnosticSpanMacroExpansion {
/* FP:json.rs-0515 */                 span: call_site,
/* FP:json.rs-0516 */                 macro_decl_name: bt.kind.descr(),
/* FP:json.rs-0517 */                 def_site_span,
/* FP:json.rs-0518 */             })
/* FP:json.rs-0519 */         });
/* FP:json.rs-0520 */ 
/* FP:json.rs-0521 */         DiagnosticSpan {
/* FP:json.rs-0522 */             file_name: sm.filename_for_diagnostics(&start.file.name).to_string(),
/* FP:json.rs-0523 */             byte_start: start.file.original_relative_byte_pos(span.lo()).0,
/* FP:json.rs-0524 */             byte_end: start.file.original_relative_byte_pos(span.hi()).0,
/* FP:json.rs-0525 */             line_start: start.line,
/* FP:json.rs-0526 */             line_end: end.line,
/* FP:json.rs-0527 */             column_start: start.col.0 + 1,
/* FP:json.rs-0528 */             column_end: end.col.0 + 1,
/* FP:json.rs-0529 */             is_primary,
/* FP:json.rs-0530 */             text: DiagnosticSpanLine::from_span(span, je),
/* FP:json.rs-0531 */             suggested_replacement: suggestion.map(|x| x.0.clone()),
/* FP:json.rs-0532 */             suggestion_applicability: suggestion.map(|x| x.1),
/* FP:json.rs-0533 */             expansion: backtrace_step,
/* FP:json.rs-0534 */             label,
/* FP:json.rs-0535 */         }
/* FP:json.rs-0536 */     }
/* FP:json.rs-0537 */ 
/* FP:json.rs-0538 */     fn from_multispan(
/* FP:json.rs-0539 */         msp: &MultiSpan,
/* FP:json.rs-0540 */         args: &FluentArgs<'_>,
/* FP:json.rs-0541 */         je: &JsonEmitter,
/* FP:json.rs-0542 */     ) -> Vec<DiagnosticSpan> {
/* FP:json.rs-0543 */         msp.span_labels()
/* FP:json.rs-0544 */             .into_iter()
/* FP:json.rs-0545 */             .map(|span_str| Self::from_span_label(span_str, None, args, je))
/* FP:json.rs-0546 */             .collect()
/* FP:json.rs-0547 */     }
/* FP:json.rs-0548 */ 
/* FP:json.rs-0549 */     fn from_suggestion(
/* FP:json.rs-0550 */         suggestion: &CodeSuggestion,
/* FP:json.rs-0551 */         args: &FluentArgs<'_>,
/* FP:json.rs-0552 */         je: &JsonEmitter,
/* FP:json.rs-0553 */     ) -> Vec<DiagnosticSpan> {
/* FP:json.rs-0554 */         suggestion
/* FP:json.rs-0555 */             .substitutions
/* FP:json.rs-0556 */             .iter()
/* FP:json.rs-0557 */             .flat_map(|substitution| {
/* FP:json.rs-0558 */                 substitution.parts.iter().map(move |suggestion_inner| {
/* FP:json.rs-0559 */                     let span_label =
/* FP:json.rs-0560 */                         SpanLabel { span: suggestion_inner.span, is_primary: true, label: None };
/* FP:json.rs-0561 */                     DiagnosticSpan::from_span_label(
/* FP:json.rs-0562 */                         span_label,
/* FP:json.rs-0563 */                         Some((&suggestion_inner.snippet, suggestion.applicability)),
/* FP:json.rs-0564 */                         args,
/* FP:json.rs-0565 */                         je,
/* FP:json.rs-0566 */                     )
/* FP:json.rs-0567 */                 })
/* FP:json.rs-0568 */             })
/* FP:json.rs-0569 */             .collect()
/* FP:json.rs-0570 */     }
/* FP:json.rs-0571 */ }
/* FP:json.rs-0572 */ 
/* FP:json.rs-0573 */ impl DiagnosticSpanLine {
/* FP:json.rs-0574 */     fn line_from_source_file(
/* FP:json.rs-0575 */         sf: &crate::rustc_span::SourceFile,
/* FP:json.rs-0576 */         index: usize,
/* FP:json.rs-0577 */         h_start: usize,
/* FP:json.rs-0578 */         h_end: usize,
/* FP:json.rs-0579 */     ) -> DiagnosticSpanLine {
/* FP:json.rs-0580 */         DiagnosticSpanLine {
/* FP:json.rs-0581 */             text: sf.get_line(index).map_or_else(String::new, |l| l.into_owned()),
/* FP:json.rs-0582 */             highlight_start: h_start,
/* FP:json.rs-0583 */             highlight_end: h_end,
/* FP:json.rs-0584 */         }
/* FP:json.rs-0585 */     }
/* FP:json.rs-0586 */ 
/* FP:json.rs-0587 */     /// Creates a list of DiagnosticSpanLines from span - each line with any part
/* FP:json.rs-0588 */     /// of `span` gets a DiagnosticSpanLine, with the highlight indicating the
/* FP:json.rs-0589 */     /// `span` within the line.
/* FP:json.rs-0590 */     fn from_span(span: Span, je: &JsonEmitter) -> Vec<DiagnosticSpanLine> {
/* FP:json.rs-0591 */         je.sm
/* FP:json.rs-0592 */             .as_ref()
/* FP:json.rs-0593 */             .and_then(|sm| {
/* FP:json.rs-0594 */                 let lines = sm.span_to_lines(span).ok()?;
/* FP:json.rs-0595 */                 // We can't get any lines if the source is unavailable.
/* FP:json.rs-0596 */                 if !should_show_source_code(
/* FP:json.rs-0597 */                     &je.ignored_directories_in_source_blocks,
/* FP:json.rs-0598 */                     &sm,
/* FP:json.rs-0599 */                     &lines.file,
/* FP:json.rs-0600 */                 ) {
/* FP:json.rs-0601 */                     return None;
/* FP:json.rs-0602 */                 }
/* FP:json.rs-0603 */ 
/* FP:json.rs-0604 */                 let sf = &*lines.file;
/* FP:json.rs-0605 */                 let span_lines = lines
/* FP:json.rs-0606 */                     .lines
/* FP:json.rs-0607 */                     .iter()
/* FP:json.rs-0608 */                     .map(|line| {
/* FP:json.rs-0609 */                         DiagnosticSpanLine::line_from_source_file(
/* FP:json.rs-0610 */                             sf,
/* FP:json.rs-0611 */                             line.line_index,
/* FP:json.rs-0612 */                             line.start_col.0 + 1,
/* FP:json.rs-0613 */                             line.end_col.0 + 1,
/* FP:json.rs-0614 */                         )
/* FP:json.rs-0615 */                     })
/* FP:json.rs-0616 */                     .collect();
/* FP:json.rs-0617 */                 Some(span_lines)
/* FP:json.rs-0618 */             })
/* FP:json.rs-0619 */             .unwrap_or_default()
/* FP:json.rs-0620 */     }
/* FP:json.rs-0621 */ }