// Generated macro for impl_129 (impl)
macro_rules! Depcrate_inferimpl_129 {
() => {
// Module: crate::infer
// Provides: {"impl_129"}
// Dependencies: {}
impl BindingMode { fn convert (annotation : BindingAnnotation) -> BindingMode { match annotation { BindingAnnotation :: Unannotated | BindingAnnotation :: Mutable => BindingMode :: Move , BindingAnnotation :: Ref => BindingMode :: Ref (Mutability :: Not) , BindingAnnotation :: RefMut => BindingMode :: Ref (Mutability :: Mut) , } } }
};
}
