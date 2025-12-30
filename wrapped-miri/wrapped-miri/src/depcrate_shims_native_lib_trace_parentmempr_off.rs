// Generated macro for mempr_off (function)
macro_rules! Depcrate_shims_native_lib_trace_parentmempr_off {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"mempr_off"}
// Dependencies: {}
# [doc = " Disables protections on the page whose address is currently in `PAGE_ADDR`."] # [doc = ""] # [doc = " SAFETY: `PAGE_ADDR` should be set to a page-aligned pointer to an owned page,"] # [doc = " `PAGE_SIZE` should be the host pagesize, and the range from `PAGE_ADDR` to"] # [doc = " `PAGE_SIZE` * `PAGE_COUNT` must be owned and allocated memory. No other threads"] # [doc = " should be running."] pub unsafe extern "C" fn mempr_off () { use std :: sync :: atomic :: Ordering ; let len = PAGE_SIZE . load (Ordering :: Relaxed) . saturating_mul (PAGE_COUNT . load (Ordering :: Relaxed)) ; unsafe { if libc :: mprotect (PAGE_ADDR . load (Ordering :: Relaxed) . cast () , len , libc :: PROT_READ | libc :: PROT_WRITE ,) != 0 { std :: process :: exit (- 1) ; } } if signal :: raise (signal :: SIGSTOP) . is_err () { std :: process :: exit (- 1) ; } }
};
}
