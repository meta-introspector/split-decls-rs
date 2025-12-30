// Generated macro for ef_e2k_flag_to_mach (function)
macro_rules! Depcrate_elfef_e2k_flag_to_mach {
() => {
// Module: crate::elf
// Provides: {"ef_e2k_flag_to_mach"}
// Dependencies: {}
# [doc = " Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`."] pub const fn ef_e2k_flag_to_mach (e_flags : u32) -> u32 { e_flags >> 24 }
};
}
