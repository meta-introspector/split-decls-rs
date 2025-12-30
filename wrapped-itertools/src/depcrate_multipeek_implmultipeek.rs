// Generated macro for MultiPeek (struct)
macro_rules! Depcrate_multipeek_implMultiPeek {
() => {
// Module: crate::multipeek_impl
// Provides: {"MultiPeek"}
// Dependencies: {}
# [doc = " See [`multipeek()`] for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct MultiPeek < I > where I : Iterator , { iter : Fuse < I > , buf : VecDeque < I :: Item > , index : usize , }
};
}
