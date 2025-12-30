// Generated macro for debug_fmt_fields (macro)
macro_rules! Depcrate_impl_macrosdebug_fmt_fields {
() => {
// Module: crate::impl_macros
// Provides: {"debug_fmt_fields"}
// Dependencies: {}
macro_rules ! debug_fmt_fields { ($ tyname : ident , $ ($ ($ field : tt) .+) ,*) => { fn fmt (& self , f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { f . debug_struct (stringify ! ($ tyname)) $ (. field (stringify ! ($ ($ field) .+) , & self .$ ($ field) .+)) * . finish () } } }
};
}
