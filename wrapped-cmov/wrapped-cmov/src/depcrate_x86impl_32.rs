// Generated macro for impl_32 (impl)
macro_rules! Depcrate_x86impl_32 {
() => {
// Module: crate::x86
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (target_arch = "x86")] impl CmovEq for u64 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { let lo = (* self & u32 :: MAX as u64) as u32 ; let hi = (* self >> 32) as u32 ; let mut tmp = 1u8 ; lo . cmovne (& ((* rhs & u32 :: MAX as u64) as u32) , 0 , & mut tmp) ; hi . cmovne (& ((* rhs >> 32) as u32) , 0 , & mut tmp) ; tmp . cmoveq (& 0 , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { let lo = (* self & u32 :: MAX as u64) as u32 ; let hi = (* self >> 32) as u32 ; let mut tmp = 1u8 ; lo . cmovne (& ((* rhs & u32 :: MAX as u64) as u32) , 0 , & mut tmp) ; hi . cmovne (& ((* rhs >> 32) as u32) , 0 , & mut tmp) ; tmp . cmoveq (& 1 , input , output) ; } }
};
}
