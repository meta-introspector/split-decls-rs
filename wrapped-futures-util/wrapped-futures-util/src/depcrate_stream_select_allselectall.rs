// Generated macro for SelectAll (struct)
macro_rules! Depcrate_stream_select_allSelectAll {
() => {
// Module: crate::stream::select_all
// Provides: {"SelectAll"}
// Dependencies: {}
# [doc = " An unbounded set of streams"] # [doc = ""] # [doc = " This \"combinator\" provides the ability to maintain a set of streams"] # [doc = " and drive them all to completion."] # [doc = ""] # [doc = " Streams are pushed into this set and their realized values are"] # [doc = " yielded as they become ready. Streams will only be polled when they"] # [doc = " generate notifications. This allows to coordinate a large number of streams."] # [doc = ""] # [doc = " Note that you can create a ready-made `SelectAll` via the"] # [doc = " `select_all` function in the `stream` module, or you can start with an"] # [doc = " empty set with the `SelectAll::new` constructor."] # [must_use = "streams do nothing unless polled"] pub struct SelectAll < St > { inner : FuturesUnordered < StreamFuture < St > > , }
};
}
