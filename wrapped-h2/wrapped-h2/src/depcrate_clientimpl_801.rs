// Generated macro for impl_801 (impl)
macro_rules! Depcrate_clientimpl_801 {
() => {
// Module: crate::client
// Provides: {"impl_801"}
// Dependencies: {}
# [cfg (feature = "unstable")] impl < B > SendRequest < B > where B : Buf , { # [doc = " Returns the number of active streams."] # [doc = ""] # [doc = " An active stream is a stream that has not yet transitioned to a closed"] # [doc = " state."] pub fn num_active_streams (& self) -> usize { self . inner . num_active_streams () } # [doc = " Returns the number of streams that are held in memory."] # [doc = ""] # [doc = " A wired stream is a stream that is either active or is closed but must"] # [doc = " stay in memory for some reason. For example, there are still outstanding"] # [doc = " userspace handles pointing to the slot."] pub fn num_wired_streams (& self) -> usize { self . inner . num_wired_streams () } }
};
}
