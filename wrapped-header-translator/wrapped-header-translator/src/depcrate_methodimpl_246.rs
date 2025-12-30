// Generated macro for impl_246 (impl)
macro_rules! Depcrate_methodimpl_246 {
() => {
// Module: crate::method
// Provides: {"impl_246"}
// Dependencies: {}
impl PropertyKind { fn parse (attrs : Option < ObjCAttributes >) -> Self { let Some (attrs) = attrs else { return Self :: Normal ; } ; let retained = attrs . retain || attrs . strong ; let unsafe_retained = attrs . assign || attrs . unsafe_retained ; match (retained , attrs . copy , attrs . weak , unsafe_retained) { (true , false , false , false) => Self :: Normal , (false , true , false , false) => Self :: Copy , (false , false , true , false) => Self :: Weak , (false , false , false , true) => Self :: UnsafeRetained , (false , false , false , false) => Self :: Normal , _ => { error ! (? attrs , "unclear property attributes") ; Self :: Normal } } } }
};
}
