// Generated macro for com_call (macro)
macro_rules! Depcrate_comcom_call {
() => {
// Module: crate::com
// Provides: {"com_call"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! com_call { ($ vtbl : ty , $ this : ident .$ method : ident ($ ($ args : tt) *)) => { ((&** ($ this . as_raw () as * mut * mut $ vtbl)) .$ method) ($ this . as_raw () , $ ($ args) *) } }
};
}
