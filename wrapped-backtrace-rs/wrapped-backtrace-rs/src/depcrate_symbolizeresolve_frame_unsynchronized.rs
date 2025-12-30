// Generated macro for resolve_frame_unsynchronized (function)
macro_rules! Depcrate_symbolizeresolve_frame_unsynchronized {
() => {
// Module: crate::symbolize
// Provides: {"resolve_frame_unsynchronized"}
// Dependencies: {}
# [doc = " Same as `resolve_frame`, only unsafe as it's unsynchronized."] # [doc = ""] # [doc = " This function does not have synchronization guarantees but is available"] # [doc = " when the `std` feature of this crate isn't compiled in. See the"] # [doc = " `resolve_frame` function for more documentation and examples."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " See information on `resolve_frame` for caveats on `cb` panicking."] pub unsafe fn resolve_frame_unsynchronized < F > (frame : & Frame , mut cb : F) where F : FnMut (& Symbol) , { unsafe { imp :: resolve (ResolveWhat :: Frame (frame) , & mut cb) } }
};
}
