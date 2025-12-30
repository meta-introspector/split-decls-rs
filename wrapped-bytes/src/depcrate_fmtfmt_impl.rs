// Generated macro for fmt_impl (macro)
macro_rules! Depcrate_fmtfmt_impl {
() => {
// Module: crate::fmt
// Provides: {"fmt_impl"}
// Dependencies: {}
macro_rules ! fmt_impl { ($ tr : ident , $ ty : ty) => { impl $ tr for $ ty { fn fmt (& self , f : & mut Formatter <'_ >) -> Result { $ tr :: fmt (& BytesRef (self . as_ref ()) , f) } } } ; }
};
}
