// Generated macro for impl_583 (impl)
macro_rules! Depcrate_interpret_operandimpl_583 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_583"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > From < ImmTy < 'tcx , Prov > > for OpTy < 'tcx , Prov > { # [inline (always)] fn from (val : ImmTy < 'tcx , Prov >) -> Self { OpTy { op : Operand :: Immediate (val . imm) , layout : val . layout } } }
};
}
