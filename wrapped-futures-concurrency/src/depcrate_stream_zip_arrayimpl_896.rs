// Generated macro for impl_896 (impl)
macro_rules! Depcrate_stream_zip_arrayimpl_896 {
() => {
// Module: crate::stream::zip::array
// Provides: {"impl_896"}
// Dependencies: {}
impl < S , const N : usize > Zip < S , N > where S : Stream , { pub (crate) fn new (streams : [S ; N]) -> Self { Self { streams , output : array :: from_fn (| _ | MaybeUninit :: uninit ()) , state : PollArray :: new_pending () , wakers : WakerArray :: new () , done : false , } } }
};
}
