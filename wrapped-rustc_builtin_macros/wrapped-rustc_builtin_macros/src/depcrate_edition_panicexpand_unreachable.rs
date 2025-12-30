// Generated macro for expand_unreachable (function)
macro_rules! Depcrate_edition_panicexpand_unreachable {
() => {
// Module: crate::edition_panic
// Provides: {"expand_unreachable"}
// Dependencies: {}
# [doc = " This expands to either"] # [doc = " - `$crate::panic::unreachable_2015!(...)` or"] # [doc = " - `$crate::panic::unreachable_2021!(...)`"] # [doc = " depending on the edition."] pub (crate) fn expand_unreachable < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let mac = if use_panic_2021 (sp) { sym :: unreachable_2021 } else { sym :: unreachable_2015 } ; expand (mac , cx , sp , tts) }
};
}
