// Generated macro for ALLOCATOR (static)
macro_rules! Depcrate_mmALLOCATOR {
() => {
// Module: crate::mm
// Provides: {"ALLOCATOR"}
// Dependencies: {}
# [cfg (target_os = "none")] # [global_allocator] pub (crate) static ALLOCATOR : Talck < RawInterruptTicketMutex , ErrOnOom > = Talc :: new (ErrOnOom) . lock () ;
};
}
