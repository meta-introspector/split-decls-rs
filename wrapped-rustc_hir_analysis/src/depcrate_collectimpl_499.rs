// Generated macro for impl_499 (impl)
macro_rules! Depcrate_collectimpl_499 {
() => {
// Module: crate::collect
// Provides: {"impl_499"}
// Dependencies: {}
impl < 'v > Visitor < 'v > for HirPlaceholderCollector { fn visit_infer (& mut self , _inf_id : HirId , inf_span : Span , kind : InferKind < 'v >) -> Self :: Result { self . spans . push (inf_span) ; if let InferKind :: Const (_) | InferKind :: Ambig (_) = kind { self . may_contain_const_infer = true ; } } }
};
}
