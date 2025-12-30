// Generated macro for HistoryBufInner (struct)
macro_rules! Depcrate_history_bufHistoryBufInner {
() => {
// Module: crate::history_buf
// Provides: {"HistoryBufInner"}
// Dependencies: {}
# [doc = " Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`HistoryBuf`] or [`HistoryBufView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct HistoryBufInner < T , S : HistoryBufStorage < T > + ? Sized > { phantom : PhantomData < T > , write_at : usize , filled : bool , data : S , }
};
}
