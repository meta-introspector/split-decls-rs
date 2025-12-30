// Generated macro for hi64 (function)
macro_rules! Depcrate_biginthi64 {
() => {
// Module: crate::bigint
// Provides: {"hi64"}
// Dependencies: {}
# [doc = " Get the high 64 bits from the vector."] # [inline (always)] pub fn hi64 (x : & [Limb]) -> (u64 , bool) { let rslc = rview (x) ; match x . len () { 0 => (0 , false) , 1 if LIMB_BITS == 32 => hi ! (@ 1 x , rslc , u32 , u32_to_hi64_1) , 1 => hi ! (@ 1 x , rslc , u64 , u64_to_hi64_1) , 2 if LIMB_BITS == 32 => hi ! (@ 2 x , rslc , u32 , u32_to_hi64_2) , 2 => hi ! (@ 2 x , rslc , u64 , u64_to_hi64_2) , _ if LIMB_BITS == 32 => hi ! (@ nonzero3 x , rslc , u32 , u32_to_hi64_3) , _ => hi ! (@ nonzero2 x , rslc , u64 , u64_to_hi64_2) , } }
};
}
