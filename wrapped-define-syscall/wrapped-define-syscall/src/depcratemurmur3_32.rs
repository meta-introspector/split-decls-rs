// Generated macro for murmur3_32 (function)
macro_rules! Depcratemurmur3_32 {
() => {
// Module: crate
// Provides: {"murmur3_32"}
// Dependencies: {}
# [cfg (any (target_feature = "static-syscalls" , all (target_arch = "bpf" , feature = "unstable-static-syscalls")))] const fn murmur3_32 (buf : & [u8] , seed : u32) -> u32 { const fn pre_mix (buf : [u8 ; 4]) -> u32 { u32 :: from_le_bytes (buf) . wrapping_mul (0xcc9e2d51) . rotate_left (15) . wrapping_mul (0x1b873593) } let mut hash = seed ; let mut i = 0 ; while i < buf . len () / 4 { let buf = [buf [i * 4] , buf [i * 4 + 1] , buf [i * 4 + 2] , buf [i * 4 + 3]] ; hash ^= pre_mix (buf) ; hash = hash . rotate_left (13) ; hash = hash . wrapping_mul (5) . wrapping_add (0xe6546b64) ; i += 1 ; } match buf . len () % 4 { 0 => { } 1 => { hash = hash ^ pre_mix ([buf [i * 4] , 0 , 0 , 0]) ; } 2 => { hash = hash ^ pre_mix ([buf [i * 4] , buf [i * 4 + 1] , 0 , 0]) ; } 3 => { hash = hash ^ pre_mix ([buf [i * 4] , buf [i * 4 + 1] , buf [i * 4 + 2] , 0]) ; } _ => { } } hash = hash ^ buf . len () as u32 ; hash = hash ^ (hash . wrapping_shr (16)) ; hash = hash . wrapping_mul (0x85ebca6b) ; hash = hash ^ (hash . wrapping_shr (13)) ; hash = hash . wrapping_mul (0xc2b2ae35) ; hash = hash ^ (hash . wrapping_shr (16)) ; hash }
};
}
