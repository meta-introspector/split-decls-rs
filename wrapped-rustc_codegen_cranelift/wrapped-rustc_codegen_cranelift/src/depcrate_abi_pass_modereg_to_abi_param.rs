// Generated macro for reg_to_abi_param (function)
macro_rules! Depcrate_abi_pass_modereg_to_abi_param {
() => {
// Module: crate::abi::pass_mode
// Provides: {"reg_to_abi_param"}
// Dependencies: {}
fn reg_to_abi_param (reg : Reg) -> AbiParam { let clif_ty = match (reg . kind , reg . size . bytes ()) { (RegKind :: Integer , 1) => types :: I8 , (RegKind :: Integer , 2) => types :: I16 , (RegKind :: Integer , 3 ..= 4) => types :: I32 , (RegKind :: Integer , 5 ..= 8) => types :: I64 , (RegKind :: Integer , 9 ..= 16) => types :: I128 , (RegKind :: Float , 2) => types :: F16 , (RegKind :: Float , 4) => types :: F32 , (RegKind :: Float , 8) => types :: F64 , (RegKind :: Float , 16) => types :: F128 , (RegKind :: Vector , size) => types :: I8 . by (u32 :: try_from (size) . unwrap ()) . unwrap () , _ => unreachable ! ("{:?}" , reg) , } ; AbiParam :: new (clif_ty) }
};
}
