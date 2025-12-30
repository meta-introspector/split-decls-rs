// Generated macro for select_all (function)
macro_rules! Depcrate_stream_select_allselect_all {
() => {
// Module: crate::stream::select_all
// Provides: {"select_all"}
// Dependencies: {}
# [doc = " Convert a list of streams into a `Stream` of results from the streams."] # [doc = ""] # [doc = " This essentially takes a list of streams (e.g. a vector, an iterator, etc.)"] # [doc = " and bundles them together into a single stream."] # [doc = " The stream will yield items as they become available on the underlying"] # [doc = " streams internally, in the order they become available."] # [doc = ""] # [doc = " Note that the returned set can also be used to dynamically push more"] # [doc = " streams into the set as they become available."] # [doc = ""] # [doc = " This function is only available when the `std` or `alloc` feature of this"] # [doc = " library is activated, and it is activated by default."] pub fn select_all < I > (streams : I) -> SelectAll < I :: Item > where I : IntoIterator , I :: Item : Stream + Unpin , { let set = SelectAll :: new () ; for stream in streams { set . push (stream) ; } assert_stream :: < < I :: Item as Stream > :: Item , _ > (set) }
};
}
