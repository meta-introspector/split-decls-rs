// Generated macro for ef_e2k_mach_to_flag (function)
macro_rules! Depcrate_elfef_e2k_mach_to_flag {
() => {
// Module: crate::elf
// Provides: {"ef_e2k_mach_to_flag"}
// Dependencies: {}
# [doc = " Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`."] pub const fn ef_e2k_mach_to_flag (e_flags : u32 , x : u32) -> u32 { (e_flags & 0xffffff) | (x << 24) }
};
}
