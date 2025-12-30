// Generated macro for impl_627 (impl)
macro_rules! Depcrate_interpret_placeimpl_627 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_627"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > OpTy < 'tcx , Prov > { # [inline (always)] pub fn as_mplace_or_imm (& self) -> Either < MPlaceTy < 'tcx , Prov > , ImmTy < 'tcx , Prov > > { match self . op () { Operand :: Indirect (mplace) => Left (MPlaceTy { mplace : * mplace , layout : self . layout }) , Operand :: Immediate (imm) => Right (ImmTy :: from_immediate (* imm , self . layout)) , } } # [inline (always)] # [cfg_attr (debug_assertions , track_caller)] pub fn assert_mem_place (& self) -> MPlaceTy < 'tcx , Prov > { self . as_mplace_or_imm () . left () . unwrap_or_else (| | { bug ! ("OpTy of type {} was immediate when it was expected to be an MPlace" , self . layout . ty) }) } }
};
}
