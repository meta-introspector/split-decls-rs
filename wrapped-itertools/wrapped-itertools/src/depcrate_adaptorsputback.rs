// Generated macro for PutBack (struct)
macro_rules! Depcrate_adaptorsPutBack {
() => {
// Module: crate::adaptors
// Provides: {"PutBack"}
// Dependencies: {}
# [derive (Clone , Debug)] # [doc = " An iterator adaptor that allows putting back a single"] # [doc = " item to the front of the iterator."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PutBack < I > where I : Iterator , { top : Option < I :: Item > , iter : I , }
};
}
