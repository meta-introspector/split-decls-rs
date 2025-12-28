macro_rules! deps {
    () => {
        BinaryHeap!();
        BinaryHeapInner!();
        Kind!();
    };
}

macro_rules! PeekMutInner {
    () => {
        deps!();
        # [doc = " Structure wrapping a mutable reference to the greatest item on a"] # [doc = " `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeapInner::peek_mut`]."] # [doc = " See its documentation for more."] pub struct PeekMutInner < 'a , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { heap : & 'a mut BinaryHeapInner < T , K , S > , sift : bool , }
    };
}

PeekMutInner!();