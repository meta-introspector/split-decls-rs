// Generated macro for kernel_start_address (function)
macro_rules! Depcrate_mmkernel_start_address {
() => {
// Module: crate::mm
// Provides: {"kernel_start_address"}
// Dependencies: {}
pub (crate) fn kernel_start_address () -> VirtAddr { KERNEL_ADDR_RANGE . start }
};
}
