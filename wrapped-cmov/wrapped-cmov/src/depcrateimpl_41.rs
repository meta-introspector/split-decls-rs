// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl CmovEq for u128 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { let lo = (* self & u64 :: MAX as u128) as u64 ; let hi = (* self >> 64) as u64 ; let mut tmp = 1u8 ; lo . cmovne (& ((* rhs & u64 :: MAX as u128) as u64) , 0 , & mut tmp) ; hi . cmovne (& ((* rhs >> 64) as u64) , 0 , & mut tmp) ; tmp . cmoveq (& 0 , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { let lo = (* self & u64 :: MAX as u128) as u64 ; let hi = (* self >> 64) as u64 ; let mut tmp = 1u8 ; lo . cmovne (& ((* rhs & u64 :: MAX as u128) as u64) , 0 , & mut tmp) ; hi . cmovne (& ((* rhs >> 64) as u64) , 0 , & mut tmp) ; tmp . cmoveq (& 1 , input , output) ; } }
};
}
