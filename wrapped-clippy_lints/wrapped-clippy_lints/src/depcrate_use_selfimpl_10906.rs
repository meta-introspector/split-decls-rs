// Generated macro for impl_10906 (impl)
macro_rules! Depcrate_use_selfimpl_10906 {
() => {
// Module: crate::use_self
// Provides: {"impl_10906"}
// Dependencies: {}
impl Visitor < '_ > for SkipTyCollector { fn visit_infer (& mut self , inf_id : HirId , _inf_span : Span , kind : InferKind < '_ >) -> Self :: Result { if let InferKind :: Ambig (_) | InferKind :: Ty (_) = kind { self . types_to_skip . push (inf_id) ; } self . visit_id (inf_id) ; } fn visit_ty (& mut self , hir_ty : & Ty < '_ , AmbigArg >) { self . types_to_skip . push (hir_ty . hir_id) ; walk_ty (self , hir_ty) ; } }
};
}
