// Generated macro for get_int_max (function)
macro_rules! Depcrate_implicit_saturating_addget_int_max {
() => {
// Module: crate::implicit_saturating_add
// Provides: {"get_int_max"}
// Dependencies: {}
fn get_int_max (ty : Ty < '_ >) -> Option < u128 > { use rustc_middle :: ty :: { Int , Uint } ; match ty . peel_refs () . kind () { Int (IntTy :: I8) => i8 :: MAX . try_into () . ok () , Int (IntTy :: I16) => i16 :: MAX . try_into () . ok () , Int (IntTy :: I32) => i32 :: MAX . try_into () . ok () , Int (IntTy :: I64) => i64 :: MAX . try_into () . ok () , Int (IntTy :: I128) => i128 :: MAX . try_into () . ok () , Int (IntTy :: Isize) => isize :: MAX . try_into () . ok () , Uint (UintTy :: U8) => Some (u8 :: MAX . into ()) , Uint (UintTy :: U16) => Some (u16 :: MAX . into ()) , Uint (UintTy :: U32) => Some (u32 :: MAX . into ()) , Uint (UintTy :: U64) => Some (u64 :: MAX . into ()) , Uint (UintTy :: U128) => Some (u128 :: MAX) , Uint (UintTy :: Usize) => usize :: MAX . try_into () . ok () , _ => None , } }
};
}
