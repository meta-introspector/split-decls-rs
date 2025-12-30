// Generated macro for HistoryBufView (type)
macro_rules! Depcrate_history_bufHistoryBufView {
() => {
// Module: crate::history_buf
// Provides: {"HistoryBufView"}
// Dependencies: {}
# [doc = " A \"view\" into a [`HistoryBuf`]"] # [doc = ""] # [doc = " Unlike [`HistoryBuf`], it doesn't have the `const N: usize` in its type signature."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use heapless::history_buf::{HistoryBuf, HistoryBufView};"] # [doc = ""] # [doc = " // Initialize a new buffer with 8 elements."] # [doc = " let mut owned_buf = HistoryBuf::<_, 8>::new();"] # [doc = " let buf: &mut HistoryBufView<_> = &mut owned_buf;"] # [doc = ""] # [doc = " // Starts with no data"] # [doc = " assert_eq!(buf.recent(), None);"] # [doc = ""] # [doc = " buf.write(3);"] # [doc = " buf.write(5);"] # [doc = " buf.extend(&[4, 4]);"] # [doc = ""] # [doc = " // The most recent written element is a four."] # [doc = " assert_eq!(buf.recent(), Some(&4));"] # [doc = ""] # [doc = " // To access all elements in an unspecified order, use `as_slice()`."] # [doc = " for el in buf.as_slice() {"] # [doc = "     println!(\"{:?}\", el);"] # [doc = " }"] # [doc = ""] # [doc = " // Now we can prepare an average of all values, which comes out to 4."] # [doc = " let avg = buf.as_slice().iter().sum::<usize>() / buf.len();"] # [doc = " assert_eq!(avg, 4);"] # [doc = " ```"] pub type HistoryBufView < T > = HistoryBufInner < T , ViewHistoryBufStorage < T > > ;
};
}
