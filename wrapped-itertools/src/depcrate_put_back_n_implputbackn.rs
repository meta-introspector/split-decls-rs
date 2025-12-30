// Generated macro for PutBackN (struct)
macro_rules! Depcrate_put_back_n_implPutBackN {
() => {
// Module: crate::put_back_n_impl
// Provides: {"PutBackN"}
// Dependencies: {}
# [doc = " An iterator adaptor that allows putting multiple"] # [doc = " items in front of the iterator."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [derive (Debug , Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PutBackN < I : Iterator > { top : Vec < I :: Item > , iter : I , }
};
}
