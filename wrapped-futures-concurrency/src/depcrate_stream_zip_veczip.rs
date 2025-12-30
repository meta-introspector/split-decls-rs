// Generated macro for Zip (struct)
macro_rules! Depcrate_stream_zip_vecZip {
() => {
// Module: crate::stream::zip::vec
// Provides: {"Zip"}
// Dependencies: {}
# [doc = " A stream that ‘zips up’ multiple streams into a single stream of pairs."] # [doc = ""] # [doc = " This `struct` is created by the [`zip`] method on the [`Zip`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`zip`]: trait.Zip.html#method.zip"] # [doc = " [`Zip`]: trait.Zip.html"] # [pin_project (PinnedDrop)] pub struct Zip < S > where S : Stream , { # [pin] streams : Vec < S > , output : Vec < MaybeUninit < < S as Stream > :: Item > > , wakers : WakerVec , state : PollVec , done : bool , len : usize , }
};
}
