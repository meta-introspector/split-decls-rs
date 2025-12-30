// Generated macro for PHYSICAL_FREE_LIST (static)
macro_rules! Depcrate_mm_physicalmemPHYSICAL_FREE_LIST {
() => {
// Module: crate::mm::physicalmem
// Provides: {"PHYSICAL_FREE_LIST"}
// Dependencies: {}
static PHYSICAL_FREE_LIST : InterruptTicketMutex < FreeList < 16 > > = InterruptTicketMutex :: new (FreeList :: new ()) ;
};
}
