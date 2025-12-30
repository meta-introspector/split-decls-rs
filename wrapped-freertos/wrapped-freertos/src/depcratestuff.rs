// Generated macro for stuff (module)
macro_rules! Depcratestuff {
() => {
// Module: crate
// Provides: {"stuff"}
// Dependencies: {}
# [cfg (target_os = "none")] mod stuff { extern crate alloc ; use core :: panic :: PanicInfo ; use cortex_m as _ ; use freertos_rust :: FreeRtosAllocator ; # [global_allocator] static GLOBAL : FreeRtosAllocator = FreeRtosAllocator ; # [cfg (needs_alloc_error_handler)] # [alloc_error_handler] fn alloc_error (_layout : alloc :: alloc :: Layout) -> ! { cortex_m :: asm :: bkpt () ; loop { } } # [panic_handler] fn panic (_info : & PanicInfo) -> ! { loop { } } }
};
}
