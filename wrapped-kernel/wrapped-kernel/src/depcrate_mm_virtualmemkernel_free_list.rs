// Generated macro for KERNEL_FREE_LIST (static)
macro_rules! Depcrate_mm_virtualmemKERNEL_FREE_LIST {
() => {
// Module: crate::mm::virtualmem
// Provides: {"KERNEL_FREE_LIST"}
// Dependencies: {}
static KERNEL_FREE_LIST : InterruptTicketMutex < FreeList < 16 > > = InterruptTicketMutex :: new (FreeList :: new ()) ;
};
}
