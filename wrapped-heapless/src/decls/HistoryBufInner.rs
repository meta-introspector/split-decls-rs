macro_rules! deps {
    () => {
        HistoryBuf!();
        HistoryBufView!();
    };
}

macro_rules! HistoryBufInner {
    () => {
        deps!();
        # [doc = " Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`HistoryBuf`] or [`HistoryBufView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct HistoryBufInner < T , S : HistoryBufStorage < T > + ? Sized > { phantom : PhantomData < T > , write_at : usize , filled : bool , data : S , }
    };
}

HistoryBufInner!()