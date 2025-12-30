// Generated macro for PeekNth (struct)
macro_rules! Depcrate_peek_nthPeekNth {
() => {
// Module: crate::peek_nth
// Provides: {"PeekNth"}
// Dependencies: {}
# [doc = " See [`peek_nth()`] for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PeekNth < I > where I : Iterator , { iter : Fuse < I > , buf : VecDeque < I :: Item > , }
};
}
