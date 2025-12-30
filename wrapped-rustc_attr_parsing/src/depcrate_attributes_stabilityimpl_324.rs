// Generated macro for impl_324 (impl)
macro_rules! Depcrate_attributes_stabilityimpl_324 {
() => {
// Module: crate::attributes::stability
// Provides: {"impl_324"}
// Dependencies: {}
impl < S : Stage > AttributeParser < S > for ConstStabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: rustc_const_stable] , template ! (List : & [r#"feature = "name""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_stability (cx , args) { this . stability = Some ((PartialConstStability { level , feature , promotable : false } , cx . attr_span ,)) ; } } ,) , (& [sym :: rustc_const_unstable] , template ! (List : & [r#"feature = "name""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((PartialConstStability { level , feature , promotable : false } , cx . attr_span ,)) ; } } ,) , (& [sym :: rustc_promotable] , template ! (Word) , | this , cx , _ | { reject_outside_std ! (cx) ; this . promotable = true ; }) ,] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (mut self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if self . promotable { if let Some ((ref mut stab , _)) = self . stability { stab . promotable = true ; } else { cx . dcx () . emit_err (session_diagnostics :: RustcPromotablePairing { span : cx . target_span }) ; } } let (stability , span) = self . stability ? ; Some (AttributeKind :: ConstStability { stability , span }) } }
};
}
