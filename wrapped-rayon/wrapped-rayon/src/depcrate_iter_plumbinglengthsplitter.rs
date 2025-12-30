// Generated macro for LengthSplitter (struct)
macro_rules! Depcrate_iter_plumbingLengthSplitter {
() => {
// Module: crate::iter::plumbing
// Provides: {"LengthSplitter"}
// Dependencies: {}
# [doc = " The length splitter is built on thief-splitting, but additionally takes"] # [doc = " into account the remaining length of the iterator."] # [derive (Clone , Copy)] struct LengthSplitter { inner : Splitter , # [doc = " The smallest we're willing to divide into.  Usually this is just 1,"] # [doc = " but you can choose a larger working size with `with_min_len()`."] min : usize , }
};
}
