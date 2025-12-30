// Generated macro for impl_584 (impl)
macro_rules! Depcrate_interpret_operandimpl_584 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_584"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > From < MPlaceTy < 'tcx , Prov > > for OpTy < 'tcx , Prov > { # [inline (always)] fn from (mplace : MPlaceTy < 'tcx , Prov >) -> Self { OpTy { op : Operand :: Indirect (* mplace . mplace ()) , layout : mplace . layout } } }
};
}
