// Generated macro for impl_888 (impl)
macro_rules! Depcrate_shareimpl_888 {
() => {
// Module: crate::share
// Provides: {"impl_888"}
// Dependencies: {}
impl StreamId { pub (crate) fn from_internal (id : crate :: frame :: StreamId) -> Self { StreamId (id . into ()) } # [doc = " Returns the `u32` corresponding to this `StreamId`"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This is the same as the `From<StreamId>` implementation, but"] # [doc = " included as an inherent method because that implementation doesn't"] # [doc = " appear in rustdocs, as well as a way to force the type instead of"] # [doc = " relying on inference."] pub fn as_u32 (& self) -> u32 { (* self) . into () } }
};
}
