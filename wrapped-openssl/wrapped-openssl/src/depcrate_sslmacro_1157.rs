// Generated macro for macro_1157 (macro)
macro_rules! Depcrate_sslmacro_1157 {
() => {
// Module: crate::ssl
// Provides: {"macro_1157"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { type SslOptionsRepr = u64 ; } else if # [cfg (any (boringssl , awslc))] { type SslOptionsRepr = u32 ; } else { type SslOptionsRepr = libc :: c_ulong ; } }
};
}
