// Generated macro for Zip (struct)
macro_rules! Depcrate_stream_zip_arrayZip {
() => {
// Module: crate::stream::zip::array
// Provides: {"Zip"}
// Dependencies: {}
# [doc = " A stream that ‘zips up’ multiple streams into a single stream of pairs."] # [doc = ""] # [doc = " This `struct` is created by the [`zip`] method on the [`Zip`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`zip`]: trait.Zip.html#method.zip"] # [doc = " [`Zip`]: trait.Zip.html"] # [pin_project (PinnedDrop)] pub struct Zip < S , const N : usize > where S : Stream , { # [pin] streams : [S ; N] , output : [MaybeUninit < < S as Stream > :: Item > ; N] , wakers : WakerArray < N > , state : PollArray < N > , done : bool , }
};
}
