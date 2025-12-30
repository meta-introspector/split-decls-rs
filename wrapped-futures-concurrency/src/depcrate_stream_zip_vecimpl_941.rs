// Generated macro for impl_941 (impl)
macro_rules! Depcrate_stream_zip_vecimpl_941 {
() => {
// Module: crate::stream::zip::vec
// Provides: {"impl_941"}
// Dependencies: {}
# [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < S > PinnedDrop for Zip < S > where S : Stream , { fn drop (self : Pin < & mut Self >) { let this = self . project () ; for (state , output) in this . state . iter_mut () . zip (this . output . iter_mut ()) { if state . is_ready () { unsafe { output . assume_init_drop () } ; } } } }
};
}
