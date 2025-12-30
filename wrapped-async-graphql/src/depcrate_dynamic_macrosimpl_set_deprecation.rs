// Generated macro for impl_set_deprecation (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_deprecation {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_deprecation"}
// Dependencies: {}
macro_rules ! impl_set_deprecation { () => { # [doc = " Set the description"] # [inline] pub fn deprecation (self , reason : Option <& str >) -> Self { Self { deprecation : Deprecation :: Deprecated { reason : reason . map (Into :: into) , } , .. self } } } ; }
};
}
