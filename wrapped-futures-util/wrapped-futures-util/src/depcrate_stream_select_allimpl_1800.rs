// Generated macro for impl_1800 (impl)
macro_rules! Depcrate_stream_select_allimpl_1800 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1800"}
// Dependencies: {}
impl < St : Stream + Unpin > SelectAll < St > { # [doc = " Constructs a new, empty `SelectAll`"] # [doc = ""] # [doc = " The returned `SelectAll` does not contain any streams and, in this"] # [doc = " state, `SelectAll::poll` will return `Poll::Ready(None)`."] pub fn new () -> Self { Self { inner : FuturesUnordered :: new () } } # [doc = " Returns the number of streams contained in the set."] # [doc = ""] # [doc = " This represents the total number of in-flight streams."] pub fn len (& self) -> usize { self . inner . len () } # [doc = " Returns `true` if the set contains no streams"] pub fn is_empty (& self) -> bool { self . inner . is_empty () } # [doc = " Push a stream into the set."] # [doc = ""] # [doc = " This function submits the given stream to the set for managing. This"] # [doc = " function will not call `poll` on the submitted stream. The caller must"] # [doc = " ensure that `SelectAll::poll` is called in order to receive task"] # [doc = " notifications."] pub fn push (& self , stream : St) { self . inner . push (stream . into_future ()) ; } # [doc = " Returns an iterator that allows inspecting each stream in the set."] pub fn iter (& self) -> Iter < '_ , St > { Iter (self . inner . iter ()) } # [doc = " Returns an iterator that allows modifying each stream in the set."] pub fn iter_mut (& mut self) -> IterMut < '_ , St > { IterMut (self . inner . iter_mut ()) } # [doc = " Clears the set, removing all streams."] pub fn clear (& mut self) { self . inner . clear () } }
};
}
