// Generated macro for impl_855 (impl)
macro_rules! Depcrate_srtpimpl_855 {
() => {
// Module: crate::srtp
// Provides: {"impl_855"}
// Dependencies: {}
impl SrtpProfileId { pub const SRTP_AES128_CM_SHA1_80 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AES128_CM_SHA1_80 as c_ulong) ; pub const SRTP_AES128_CM_SHA1_32 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AES128_CM_SHA1_32 as c_ulong) ; pub const SRTP_AES128_F8_SHA1_80 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AES128_F8_SHA1_80 as c_ulong) ; pub const SRTP_AES128_F8_SHA1_32 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AES128_F8_SHA1_32 as c_ulong) ; pub const SRTP_NULL_SHA1_80 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_NULL_SHA1_80 as c_ulong) ; pub const SRTP_NULL_SHA1_32 : SrtpProfileId = SrtpProfileId (ffi :: SRTP_NULL_SHA1_32 as c_ulong) ; # [cfg (any (boringssl , ossl110 , awslc))] pub const SRTP_AEAD_AES_128_GCM : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AEAD_AES_128_GCM as c_ulong) ; # [cfg (any (boringssl , ossl110 , awslc))] pub const SRTP_AEAD_AES_256_GCM : SrtpProfileId = SrtpProfileId (ffi :: SRTP_AEAD_AES_256_GCM as c_ulong) ; # [doc = " Creates a `SrtpProfileId` from an integer representation."] pub fn from_raw (value : c_ulong) -> SrtpProfileId { SrtpProfileId (value) } # [doc = " Returns the integer representation of `SrtpProfileId`."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_ulong { self . 0 } }
};
}
