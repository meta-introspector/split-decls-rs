// Generated macro for from_u64 (function)
macro_rules! Depcrate_bigintfrom_u64 {
() => {
// Module: crate::bigint
// Provides: {"from_u64"}
// Dependencies: {}
# [doc = " Create StackVec from u64 value."] # [inline (always)] # [allow (clippy :: branches_sharing_code)] pub fn from_u64 (x : u64) -> VecType { let mut vec = VecType :: new () ; debug_assert ! (vec . capacity () >= 2) ; if LIMB_BITS == 32 { vec . try_push (x as Limb) . unwrap () ; vec . try_push ((x >> 32) as Limb) . unwrap () ; } else { vec . try_push (x as Limb) . unwrap () ; } vec . normalize () ; vec }
};
}
