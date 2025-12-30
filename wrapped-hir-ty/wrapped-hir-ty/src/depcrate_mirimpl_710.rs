// Generated macro for impl_710 (impl)
macro_rules! Depcrate_mirimpl_710 {
() => {
// Module: crate::mir
// Provides: {"impl_710"}
// Dependencies: {}
impl MirSpan { pub fn is_ref_span (& self , body : & Body) -> bool { match * self { MirSpan :: ExprId (expr) => matches ! (body [expr] , Expr :: Ref { .. }) , MirSpan :: BindingId (binding) => { matches ! (body [binding] . mode , BindingAnnotation :: Ref | BindingAnnotation :: RefMut) } MirSpan :: PatId (_) | MirSpan :: SelfParam | MirSpan :: Unknown => false , } } }
};
}
