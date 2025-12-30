// Generated macro for max_int_num (function)
macro_rules! Depcrate_operators_arithmetic_side_effectsmax_int_num {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"max_int_num"}
// Dependencies: {}
fn max_int_num (ty : Ty < '_ >) -> Option < u128 > { match ty . peel_refs () . kind () { ty :: Uint (UintTy :: U8) => Some (u8 :: MAX . into ()) , ty :: Uint (UintTy :: U16) => Some (u16 :: MAX . into ()) , ty :: Uint (UintTy :: U32) => Some (u32 :: MAX . into ()) , ty :: Uint (UintTy :: U64) => Some (u64 :: MAX . into ()) , ty :: Uint (UintTy :: U128) => Some (u128 :: MAX) , ty :: Uint (UintTy :: Usize) => usize :: MAX . try_into () . ok () , _ => None , } }
};
}
