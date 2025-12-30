// Generated macro for impl_212 (impl)
macro_rules! Depcrate_hirimpl_212 {
() => {
// Module: crate::hir
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'hir > PathSegment < 'hir > { # [doc = " Converts an identifier to the corresponding segment."] pub fn new (ident : Ident , hir_id : HirId , res : Res) -> PathSegment < 'hir > { PathSegment { ident , hir_id , res , infer_args : true , args : None } } pub fn invalid () -> Self { Self :: new (Ident :: dummy () , HirId :: INVALID , Res :: Err) } pub fn args (& self) -> & GenericArgs < 'hir > { if let Some (ref args) = self . args { args } else { const DUMMY : & GenericArgs < '_ > = & GenericArgs :: none () ; DUMMY } } }
};
}
