// Generated macro for impl_1662 (impl)
macro_rules! Depcrate_stream_unfoldimpl_1662 {
() => {
// Module: crate::stream::unfold
// Provides: {"impl_1662"}
// Dependencies: {}
impl < T , F , Fut , Item > FusedStream for Unfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Option < (Item , T) > > , { fn is_terminated (& self) -> bool { matches ! (self . state , UnfoldState :: Empty) } }
};
}
