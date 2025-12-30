// Generated macro for kernel_heap_end (function)
macro_rules! Depcrate_mm_virtualmemkernel_heap_end {
() => {
// Module: crate::mm::virtualmem
// Provides: {"kernel_heap_end"}
// Dependencies: {}
# [doc = " End of the virtual memory address space reserved for kernel memory (inclusive)."] # [doc = " The virtual memory address space reserved for the task heap starts after this."] # [inline] pub fn kernel_heap_end () -> VirtAddr { cfg_if :: cfg_if ! { if # [cfg (target_arch = "aarch64")] { VirtAddr :: new (0xFFFF_FFFF_FFFF) } else if # [cfg (target_arch = "riscv64")] { VirtAddr :: new (0x0040_0000_0000 - 1) } else if # [cfg (target_arch = "x86_64")] { use x86_64 :: structures :: paging :: PageTableIndex ; let p4_index = if cfg ! (feature = "common-os") { PageTableIndex :: new (1) } else { PageTableIndex :: new (256) } ; let addr = u64 :: from (p4_index) << 39 ; assert_eq ! (VirtAddr :: new_truncate (addr) . p4_index () , p4_index) ; VirtAddr :: new_truncate (addr - 1) } } }
};
}
