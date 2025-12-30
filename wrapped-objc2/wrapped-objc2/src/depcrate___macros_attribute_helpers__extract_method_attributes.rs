// Generated macro for __extract_method_attributes (macro)
macro_rules! Depcrate___macros_attribute_helpers__extract_method_attributes {
() => {
// Module: crate::__macros::attribute_helpers
// Provides: {"__extract_method_attributes"}
// Dependencies: {}
# [doc = " Extract our custom method attributes, and send it to another macro."] # [doc = ""] # [doc = " Handles:"] # [doc = " - `#[unsafe(method(...))]` or `#[unsafe(method_id(...))]`."] # [doc = " - `#[unsafe(method_family(...))]`."] # [doc = " - `#[optional]`."] # [doc = ""] # [doc = " This will ensure that there is one and only one of the `method` attributes"] # [doc = " present."] # [doc (hidden)] # [macro_export] macro_rules ! __extract_method_attributes { { ($ ($ m : tt) *) ($ out_macro : path) $ ($ out_args : tt) * } => { $ crate :: __extract_method_attributes_inner ! { ($ ($ m) *) () () () () () ($ out_macro) $ ($ out_args) * } } ; }
};
}
