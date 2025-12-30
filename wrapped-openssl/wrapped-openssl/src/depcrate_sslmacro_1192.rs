// Generated macro for macro_1192 (macro)
macro_rules! Depcrate_sslmacro_1192 {
() => {
// Module: crate::ssl
// Provides: {"macro_1192"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , awslc))] { type SslCacheTy = i64 ; type SslCacheSize = libc :: c_ulong ; type MtuTy = u32 ; type SizeTy = usize ; } else { type SslCacheTy = i64 ; type SslCacheSize = c_long ; type MtuTy = c_long ; type SizeTy = u32 ; } }
};
}
