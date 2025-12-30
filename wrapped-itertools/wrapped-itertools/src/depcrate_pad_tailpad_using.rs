// Generated macro for pad_using (function)
macro_rules! Depcrate_pad_tailpad_using {
() => {
// Module: crate::pad_tail
// Provides: {"pad_using"}
// Dependencies: {}
# [doc = " Create a new `PadUsing` iterator."] pub fn pad_using < I , F > (iter : I , min : usize , filler : F) -> PadUsing < I , F > where I : Iterator , F : FnMut (usize) -> I :: Item , { PadUsing { iter : iter . fuse () , min , pos : 0 , filler , } }
};
}
