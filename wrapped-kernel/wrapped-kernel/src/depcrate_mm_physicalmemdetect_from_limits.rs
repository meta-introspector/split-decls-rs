// Generated macro for detect_from_limits (function)
macro_rules! Depcrate_mm_physicalmemdetect_from_limits {
() => {
// Module: crate::mm::physicalmem
// Provides: {"detect_from_limits"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "riscv64"))] unsafe fn detect_from_limits () -> Result < () , () > { let limit = crate :: arch :: kernel :: get_limit () ; if limit == 0 { return Err (()) ; } # [cfg (target_arch = "riscv64")] let ram_address = crate :: arch :: kernel :: get_ram_address () . as_usize () ; # [cfg (target_arch = "aarch64")] let ram_address = 0 ; let range = PageRange :: new (super :: kernel_end_address () . as_usize () , ram_address + limit) . unwrap () ; unsafe { PHYSICAL_FREE_LIST . lock () . deallocate (range) . unwrap () ; map_frame_range (range) ; } TOTAL_MEMORY . fetch_add (range . len () . get () , Ordering :: Relaxed) ; Ok (()) }
};
}
