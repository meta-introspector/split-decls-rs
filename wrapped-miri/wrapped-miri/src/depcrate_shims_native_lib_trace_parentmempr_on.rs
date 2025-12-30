// Generated macro for mempr_on (function)
macro_rules! Depcrate_shims_native_lib_trace_parentmempr_on {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"mempr_on"}
// Dependencies: {}
# [doc = " Reenables protection on the page set by `PAGE_ADDR`."] # [doc = ""] # [doc = " SAFETY: See `mempr_off()`."] pub unsafe extern "C" fn mempr_on () { use std :: sync :: atomic :: Ordering ; let len = PAGE_SIZE . load (Ordering :: Relaxed) . wrapping_mul (PAGE_COUNT . load (Ordering :: Relaxed)) ; unsafe { if libc :: mprotect (PAGE_ADDR . load (Ordering :: Relaxed) . cast () , len , libc :: PROT_NONE) != 0 { std :: process :: exit (- 1) ; } } if signal :: raise (signal :: SIGSTOP) . is_err () { std :: process :: exit (- 1) ; } }
};
}
