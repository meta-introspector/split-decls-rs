// Generated macro for impl_331 (impl)
macro_rules! Depcrate_hirimpl_331 {
() => {
// Module: crate::hir
// Provides: {"impl_331"}
// Dependencies: {}
impl BindingAnnotation { pub fn new (is_mutable : bool , is_ref : bool) -> Self { match (is_mutable , is_ref) { (true , true) => BindingAnnotation :: RefMut , (false , true) => BindingAnnotation :: Ref , (true , false) => BindingAnnotation :: Mutable , (false , false) => BindingAnnotation :: Unannotated , } } }
};
}
