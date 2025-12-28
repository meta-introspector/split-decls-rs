macro_rules! deps {
    () => {
        BinaryHeapView!();
        VecInner!();
        BinaryHeap!();
    };
}

macro_rules! BinaryHeapInner {
    () => {
        deps!();
        # [doc = " Base struct for [`BinaryHeap`] and [`BinaryHeapView`], generic over the [`VecStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`BinaryHeap`] or [`BinaryHeapView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "T: Zeroize, S: Zeroize"))] pub struct BinaryHeapInner < T , K , S : VecStorage < T > + ? Sized > { pub (crate) _kind : PhantomData < K > , pub (crate) data : VecInner < T , usize , S > , }
    };
}

BinaryHeapInner!();