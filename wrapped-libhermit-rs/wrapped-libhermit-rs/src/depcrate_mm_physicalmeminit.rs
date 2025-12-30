// Generated macro for init (function)
macro_rules! Depcrate_mm_physicalmeminit {
() => {
// Module: crate::mm::physicalmem
// Provides: {"init"}
// Dependencies: {}
unsafe fn init () { if env :: is_uefi () && DeviceAlloc . phys_offset () != VirtAddr :: zero () { let start = DeviceAlloc . phys_offset () ; let count = DeviceAlloc . phys_offset () . as_u64 () / HugePageSize :: SIZE ; let count = usize :: try_from (count) . unwrap () ; paging :: unmap :: < HugePageSize > (start , count) ; } if let Err (_err) = unsafe { detect_from_fdt () } { cfg_if :: cfg_if ! { if # [cfg (any (target_arch = "aarch64" , target_arch = "riscv64"))] { error ! ("Could not detect physical memory from FDT") ; unsafe { detect_from_limits () . unwrap () ; } } else { panic ! ("Could not detect physical memory from FDT") ; } } } }
};
}
