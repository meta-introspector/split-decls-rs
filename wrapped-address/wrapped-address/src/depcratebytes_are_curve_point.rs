// Generated macro for bytes_are_curve_point (function)
macro_rules! Depcratebytes_are_curve_point {
() => {
// Module: crate
// Provides: {"bytes_are_curve_point"}
// Dependencies: {}
# [cfg (any (target_os = "solana" , target_arch = "bpf" , feature = "curve25519"))] # [allow (clippy :: used_underscore_binding)] pub fn bytes_are_curve_point < T : AsRef < [u8] > > (_bytes : T) -> bool { # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] { let Ok (compressed_edwards_y) = curve25519_dalek :: edwards :: CompressedEdwardsY :: from_slice (_bytes . as_ref ()) else { return false ; } ; compressed_edwards_y . decompress () . is_some () } # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unimplemented ! () ; }
};
}
