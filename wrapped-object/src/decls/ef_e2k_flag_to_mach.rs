macro_rules! ef_e2k_flag_to_mach {
    () => {
        # [doc = " Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`."] pub const fn ef_e2k_flag_to_mach (e_flags : u32) -> u32 { e_flags >> 24 }
    };
}

ef_e2k_flag_to_mach!()