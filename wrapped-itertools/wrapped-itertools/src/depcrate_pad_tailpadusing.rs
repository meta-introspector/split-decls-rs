// Generated macro for PadUsing (struct)
macro_rules! Depcrate_pad_tailPadUsing {
() => {
// Module: crate::pad_tail
// Provides: {"PadUsing"}
// Dependencies: {}
# [doc = " An iterator adaptor that pads a sequence to a minimum length by filling"] # [doc = " missing elements using a closure."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [doc = ""] # [doc = " See [`.pad_using()`](crate::Itertools::pad_using) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PadUsing < I , F > { iter : Fuse < I > , min : usize , pos : usize , filler : F , }
};
}
