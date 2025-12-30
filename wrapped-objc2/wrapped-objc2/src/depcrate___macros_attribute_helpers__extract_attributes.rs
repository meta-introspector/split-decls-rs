// Generated macro for __extract_attributes (macro)
macro_rules! Depcrate___macros_attribute_helpers__extract_attributes {
() => {
// Module: crate::__macros::attribute_helpers
// Provides: {"__extract_attributes"}
// Dependencies: {}
# [doc = " Extract custom attributes, and forward them to another macro."] # [doc = ""] # [doc = " Used by `define_class!`, `extern_class!` and `extern_protocol!`."] # [doc = ""] # [doc = " This will ensure that there is only one of our custom attributes present."] # [doc = ""] # [doc = " Custom attributes support both invocation forms `#[a = b]` and `#[a(b)]`,"] # [doc = " and if they are wrapped in `unsafe`, that token will be passed onwards."] # [doc (hidden)] # [macro_export] macro_rules ! __extract_attributes { { ($ ($ attrs : tt) *) ($ out_macro : path) $ ($ out_args : tt) * } => { $ crate :: __extract_attributes_inner ! { ($ ($ attrs) *) () () () () () () ($ out_macro) $ ($ out_args) * } } ; }
};
}
