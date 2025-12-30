// Generated macro for MemEvents (struct)
macro_rules! Depcrate_shims_native_libMemEvents {
() => {
// Module: crate::shims::native_lib
// Provides: {"MemEvents"}
// Dependencies: {}
# [doc = " The final results of an FFI trace, containing every relevant event detected"] # [doc = " by the tracer."] # [derive (Serialize , Deserialize , Debug)] pub struct MemEvents { # [doc = " An list of memory accesses that occurred, in the order they occurred in."] pub acc_events : Vec < AccessEvent > , }
};
}
