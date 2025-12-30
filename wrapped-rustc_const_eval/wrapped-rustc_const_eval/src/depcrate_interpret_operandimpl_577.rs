// Generated macro for impl_577 (impl)
macro_rules! Depcrate_interpret_operandimpl_577 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_577"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > std :: ops :: Deref for ImmTy < 'tcx , Prov > { type Target = Immediate < Prov > ; # [inline (always)] fn deref (& self) -> & Immediate < Prov > { & self . imm } }
};
}
