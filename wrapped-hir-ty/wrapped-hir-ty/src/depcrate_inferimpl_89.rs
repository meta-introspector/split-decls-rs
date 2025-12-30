// Generated macro for impl_89 (impl)
macro_rules! Depcrate_inferimpl_89 {
() => {
// Module: crate::infer
// Provides: {"impl_89"}
// Dependencies: {}
impl BindingMode { fn convert (annotation : BindingAnnotation) -> BindingMode { match annotation { BindingAnnotation :: Unannotated | BindingAnnotation :: Mutable => BindingMode :: Move , BindingAnnotation :: Ref => BindingMode :: Ref (Mutability :: Not) , BindingAnnotation :: RefMut => BindingMode :: Ref (Mutability :: Mut) , } } }
};
}
