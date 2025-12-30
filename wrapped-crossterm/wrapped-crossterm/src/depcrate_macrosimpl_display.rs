// Generated macro for impl_display (macro)
macro_rules! Depcrate_macrosimpl_display {
() => {
// Module: crate::macros
// Provides: {"impl_display"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! impl_display { (for $ ($ t : ty) ,+) => { $ (impl :: std :: fmt :: Display for $ t { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { $ crate :: command :: execute_fmt (f , self) } }) * } }
};
}
