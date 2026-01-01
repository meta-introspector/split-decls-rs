/* FP:translation.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_translation_UNPARSEABLE_0001
/* FP:translation.rs-0002 */ use std::borrow::Cow;
/* FP:translation.rs-0003 */ use std::env;
/* FP:translation.rs-0004 */ use std::error::Report;
/* FP:translation.rs-0005 */ use std::sync::Arc;
/* FP:translation.rs-0006 */ 
/* FP:translation.rs-0007 */ pub use crate::rustc_error_messages::{FluentArgs, LazyFallbackBundle};
/* FP:translation.rs-0008 */ use tracing::{debug, trace};
/* FP:translation.rs-0009 */ 
/* FP:translation.rs-0010 */ use crate::error::{TranslateError, TranslateErrorKind};
/* FP:translation.rs-0011 */ use crate::snippet::Style;
/* FP:translation.rs-0012 */ use crate::{DiagArg, DiagMessage, FluentBundle};
/* FP:translation.rs-0013 */ 
/* FP:translation.rs-0014 */ /// Convert diagnostic arguments (a rustc internal type that exists to implement
/* FP:translation.rs-0015 */ /// `Encodable`/`Decodable`) into `FluentArgs` which is necessary to perform translation.
/* FP:translation.rs-0016 */ ///
/* FP:translation.rs-0017 */ /// Typically performed once for each diagnostic at the start of `emit_diagnostic` and then
/* FP:translation.rs-0018 */ /// passed around as a reference thereafter.
/* FP:translation.rs-0019 */ pub fn to_fluent_args<'iter>(iter: impl Iterator<Item = DiagArg<'iter>>) -> FluentArgs<'static> {
/* FP:translation.rs-0020 */     let mut args = if let Some(size) = iter.size_hint().1 {
/* FP:translation.rs-0021 */         FluentArgs::with_capacity(size)
/* FP:translation.rs-0022 */     } else {
/* FP:translation.rs-0023 */         FluentArgs::new()
/* FP:translation.rs-0024 */     };
/* FP:translation.rs-0025 */ 
/* FP:translation.rs-0026 */     for (k, v) in iter {
/* FP:translation.rs-0027 */         args.set(k.clone(), v.clone());
/* FP:translation.rs-0028 */     }
/* FP:translation.rs-0029 */ 
/* FP:translation.rs-0030 */     args
/* FP:translation.rs-0031 */ }
/* FP:translation.rs-0032 */ 
/* FP:translation.rs-0033 */ #[derive(Clone)]
/* FP:translation.rs-0034 */ pub struct Translator {
/* FP:translation.rs-0035 */     /// Localized diagnostics for the locale requested by the user. If no language was requested by
/* FP:translation.rs-0036 */     /// the user then this will be `None` and `fallback_fluent_bundle` should be used.
/* FP:translation.rs-0037 */     pub fluent_bundle: Option<Arc<FluentBundle>>,
/* FP:translation.rs-0038 */     /// Return `FluentBundle` with localized diagnostics for the default locale of the compiler.
/* FP:translation.rs-0039 */     /// Used when the user has not requested a specific language or when a localized diagnostic is
/* FP:translation.rs-0040 */     /// unavailable for the requested locale.
/* FP:translation.rs-0041 */     pub fallback_fluent_bundle: LazyFallbackBundle,
/* FP:translation.rs-0042 */ }
/* FP:translation.rs-0043 */ 
/* FP:translation.rs-0044 */ impl Translator {
/* FP:translation.rs-0045 */     pub fn with_fallback_bundle(
/* FP:translation.rs-0046 */         resources: Vec<&'static str>,
/* FP:translation.rs-0047 */         with_directionality_markers: bool,
/* FP:translation.rs-0048 */     ) -> Translator {
/* FP:translation.rs-0049 */         Translator {
/* FP:translation.rs-0050 */             fluent_bundle: None,
/* FP:translation.rs-0051 */             fallback_fluent_bundle: crate::fallback_fluent_bundle(
/* FP:translation.rs-0052 */                 resources,
/* FP:translation.rs-0053 */                 with_directionality_markers,
/* FP:translation.rs-0054 */             ),
/* FP:translation.rs-0055 */         }
/* FP:translation.rs-0056 */     }
/* FP:translation.rs-0057 */ 
/* FP:translation.rs-0058 */     /// Convert `DiagMessage`s to a string, performing translation if necessary.
/* FP:translation.rs-0059 */     pub fn translate_messages(
/* FP:translation.rs-0060 */         &self,
/* FP:translation.rs-0061 */         messages: &[(DiagMessage, Style)],
/* FP:translation.rs-0062 */         args: &FluentArgs<'_>,
/* FP:translation.rs-0063 */     ) -> Cow<'_, str> {
/* FP:translation.rs-0064 */         Cow::Owned(
/* FP:translation.rs-0065 */             messages
/* FP:translation.rs-0066 */                 .iter()
/* FP:translation.rs-0067 */                 .map(|(m, _)| self.translate_message(m, args).map_err(Report::new).unwrap())
/* FP:translation.rs-0068 */                 .collect::<String>(),
/* FP:translation.rs-0069 */         )
/* FP:translation.rs-0070 */     }
/* FP:translation.rs-0071 */ 
/* FP:translation.rs-0072 */     /// Convert a `DiagMessage` to a string, performing translation if necessary.
/* FP:translation.rs-0073 */     pub fn translate_message<'a>(
/* FP:translation.rs-0074 */         &'a self,
/* FP:translation.rs-0075 */         message: &'a DiagMessage,
/* FP:translation.rs-0076 */         args: &'a FluentArgs<'_>,
/* FP:translation.rs-0077 */     ) -> Result<Cow<'a, str>, TranslateError<'a>> {
/* FP:translation.rs-0078 */         trace!(?message, ?args);
/* FP:translation.rs-0079 */         let (identifier, attr) = match message {
/* FP:translation.rs-0080 */             DiagMessage::Str(msg) | DiagMessage::Translated(msg) => {
/* FP:translation.rs-0081 */                 return Ok(Cow::Borrowed(msg));
/* FP:translation.rs-0082 */             }
/* FP:translation.rs-0083 */             DiagMessage::FluentIdentifier(identifier, attr) => (identifier, attr),
/* FP:translation.rs-0084 */         };
/* FP:translation.rs-0085 */         let translate_with_bundle =
/* FP:translation.rs-0086 */             |bundle: &'a FluentBundle| -> Result<Cow<'_, str>, TranslateError<'_>> {
/* FP:translation.rs-0087 */                 let message = bundle
/* FP:translation.rs-0088 */                     .get_message(identifier)
/* FP:translation.rs-0089 */                     .ok_or(TranslateError::message(identifier, args))?;
/* FP:translation.rs-0090 */                 let value = match attr {
/* FP:translation.rs-0091 */                     Some(attr) => message
/* FP:translation.rs-0092 */                         .get_attribute(attr)
/* FP:translation.rs-0093 */                         .ok_or(TranslateError::attribute(identifier, args, attr))?
/* FP:translation.rs-0094 */                         .value(),
/* FP:translation.rs-0095 */                     None => message.value().ok_or(TranslateError::value(identifier, args))?,
/* FP:translation.rs-0096 */                 };
/* FP:translation.rs-0097 */                 debug!(?message, ?value);
/* FP:translation.rs-0098 */ 
/* FP:translation.rs-0099 */                 let mut errs = vec![];
/* FP:translation.rs-0100 */                 let translated = bundle.format_pattern(value, Some(args), &mut errs);
/* FP:translation.rs-0101 */                 debug!(?translated, ?errs);
/* FP:translation.rs-0102 */                 if errs.is_empty() {
/* FP:translation.rs-0103 */                     Ok(translated)
/* FP:translation.rs-0104 */                 } else {
/* FP:translation.rs-0105 */                     Err(TranslateError::fluent(identifier, args, errs))
/* FP:translation.rs-0106 */                 }
/* FP:translation.rs-0107 */             };
/* FP:translation.rs-0108 */ 
/* FP:translation.rs-0109 */         try {
/* FP:translation.rs-0110 */             match self.fluent_bundle.as_ref().map(|b| translate_with_bundle(b)) {
/* FP:translation.rs-0111 */                 // The primary bundle was present and translation succeeded
/* FP:translation.rs-0112 */                 Some(Ok(t)) => t,
/* FP:translation.rs-0113 */ 
/* FP:translation.rs-0114 */                 // If `translate_with_bundle` returns `Err` with the primary bundle, this is likely
/* FP:translation.rs-0115 */                 // just that the primary bundle doesn't contain the message being translated, so
/* FP:translation.rs-0116 */                 // proceed to the fallback bundle.
/* FP:translation.rs-0117 */                 Some(Err(
/* FP:translation.rs-0118 */                     primary @ TranslateError::One {
/* FP:translation.rs-0119 */                         kind: TranslateErrorKind::MessageMissing, ..
/* FP:translation.rs-0120 */                     },
/* FP:translation.rs-0121 */                 )) => translate_with_bundle(&self.fallback_fluent_bundle)
/* FP:translation.rs-0122 */                     .map_err(|fallback| primary.and(fallback))?,
/* FP:translation.rs-0123 */ 
/* FP:translation.rs-0124 */                 // Always yeet out for errors on debug (unless
/* FP:translation.rs-0125 */                 // `RUSTC_TRANSLATION_NO_DEBUG_ASSERT` is set in the environment - this allows
/* FP:translation.rs-0126 */                 // local runs of the test suites, of builds with debug assertions, to test the
/* FP:translation.rs-0127 */                 // behaviour in a normal build).
/* FP:translation.rs-0128 */                 Some(Err(primary))
/* FP:translation.rs-0129 */                     if cfg!(debug_assertions)
/* FP:translation.rs-0130 */                         && env::var("RUSTC_TRANSLATION_NO_DEBUG_ASSERT").is_err() =>
/* FP:translation.rs-0131 */                 {
/* FP:translation.rs-0132 */                     do yeet primary
/* FP:translation.rs-0133 */                 }
/* FP:translation.rs-0134 */ 
/* FP:translation.rs-0135 */                 // ..otherwise, for end users, an error about this wouldn't be useful or actionable, so
/* FP:translation.rs-0136 */                 // just hide it and try with the fallback bundle.
/* FP:translation.rs-0137 */                 Some(Err(primary)) => translate_with_bundle(&self.fallback_fluent_bundle)
/* FP:translation.rs-0138 */                     .map_err(|fallback| primary.and(fallback))?,
/* FP:translation.rs-0139 */ 
/* FP:translation.rs-0140 */                 // The primary bundle is missing, proceed to the fallback bundle
/* FP:translation.rs-0141 */                 None => translate_with_bundle(&self.fallback_fluent_bundle)
/* FP:translation.rs-0142 */                     .map_err(|fallback| TranslateError::primary(identifier, args).and(fallback))?,
/* FP:translation.rs-0143 */             }
/* FP:translation.rs-0144 */         }
/* FP:translation.rs-0145 */     }
/* FP:translation.rs-0146 */ }