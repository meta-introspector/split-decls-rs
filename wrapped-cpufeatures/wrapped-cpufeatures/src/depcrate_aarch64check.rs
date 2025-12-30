// Generated macro for check (macro)
macro_rules! Depcrate_aarch64check {
() => {
// Module: crate::aarch64
// Provides: {"check"}
// Dependencies: {}
# [cfg (target_vendor = "apple")] # [macro_export] # [doc (hidden)] macro_rules ! check { ("aes") => { true } ; ("dit") => { unsafe { $ crate :: aarch64 :: sysctlbyname (b"hw.optional.arm.FEAT_DIT\0") } } ; ("sha2") => { true } ; ("sha3") => { unsafe { $ crate :: aarch64 :: sysctlbyname (b"hw.optional.armv8_2_sha512\0") && $ crate :: aarch64 :: sysctlbyname (b"hw.optional.armv8_2_sha3\0") } } ; ("sm4") => { false } ; }
};
}
