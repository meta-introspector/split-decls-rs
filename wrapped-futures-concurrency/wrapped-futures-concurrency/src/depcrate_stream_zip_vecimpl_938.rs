// Generated macro for impl_938 (impl)
macro_rules! Depcrate_stream_zip_vecimpl_938 {
() => {
// Module: crate::stream::zip::vec
// Provides: {"impl_938"}
// Dependencies: {}
impl < S > Zip < S > where S : Stream , { pub (crate) fn new (streams : Vec < S >) -> Self { let len = streams . len () ; Self { len , streams , wakers : WakerVec :: new (len) , output : (0 .. len) . map (| _ | MaybeUninit :: uninit ()) . collect () , state : PollVec :: new_pending (len) , done : false , } } }
};
}
