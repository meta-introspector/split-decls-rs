// Generated macro for consts (module)
macro_rules! Depcrate_compressionconsts {
() => {
// Module: crate::compression
// Provides: {"consts"}
// Dependencies: {}
mod consts { use crate :: LE_FLAG ; pub const ALT_BN128_G1_COMPRESS_BE : u64 = 0 ; pub const ALT_BN128_G1_DECOMPRESS_BE : u64 = 1 ; pub const ALT_BN128_G2_COMPRESS_BE : u64 = 2 ; pub const ALT_BN128_G2_DECOMPRESS_BE : u64 = 3 ; # [deprecated (since = "3.1.0" , note = "Please use `ALT_BN128_G1_COMPRESS_BE` instead")] pub const ALT_BN128_G1_COMPRESS : u64 = ALT_BN128_G1_COMPRESS_BE ; # [deprecated (since = "3.1.0" , note = "Please use `ALT_BN128_G1_DECOMPRESS_BE` instead")] pub const ALT_BN128_G1_DECOMPRESS : u64 = ALT_BN128_G1_DECOMPRESS_BE ; # [deprecated (since = "3.1.0" , note = "Please use `ALT_BN128_G2_COMPRESS_BE` instead")] pub const ALT_BN128_G2_COMPRESS : u64 = ALT_BN128_G2_COMPRESS_BE ; # [deprecated (since = "3.1.0" , note = "Please use `ALT_BN128_G2_DECOMPRESS_BE` instead")] pub const ALT_BN128_G2_DECOMPRESS : u64 = ALT_BN128_G2_DECOMPRESS_BE ; pub const ALT_BN128_G1_COMPRESS_LE : u64 = ALT_BN128_G1_COMPRESS_BE | LE_FLAG ; pub const ALT_BN128_G1_DECOMPRESS_LE : u64 = ALT_BN128_G1_DECOMPRESS_BE | LE_FLAG ; pub const ALT_BN128_G2_COMPRESS_LE : u64 = ALT_BN128_G2_COMPRESS_BE | LE_FLAG ; pub const ALT_BN128_G2_DECOMPRESS_LE : u64 = ALT_BN128_G2_DECOMPRESS_BE | LE_FLAG ; }
};
}
