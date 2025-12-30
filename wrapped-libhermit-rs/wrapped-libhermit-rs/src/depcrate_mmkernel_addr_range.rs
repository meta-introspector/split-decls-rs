// Generated macro for KERNEL_ADDR_RANGE (static)
macro_rules! Depcrate_mmKERNEL_ADDR_RANGE {
() => {
// Module: crate::mm
// Provides: {"KERNEL_ADDR_RANGE"}
// Dependencies: {}
# [doc = " Physical and virtual address range of the 2 MiB pages that map the kernel."] static KERNEL_ADDR_RANGE : Lazy < Range < VirtAddr > > = Lazy :: new (| | { if cfg ! (target_os = "none") { env :: get_base_address () . align_down (LargePageSize :: SIZE) .. (env :: get_base_address () + env :: get_image_size ()) . align_up (LargePageSize :: SIZE) } else { VirtAddr :: zero () .. VirtAddr :: zero () } }) ;
};
}
