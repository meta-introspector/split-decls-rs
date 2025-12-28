macro_rules! deps {
    () => {
        PeekMutInner!();
        BinaryHeap!();
    };
}

macro_rules! PeekMut {
    () => {
        deps!();
        # [doc = " Structure wrapping a mutable reference to the greatest item on a"] # [doc = " `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::peek_mut`]."] # [doc = " See its documentation for more."] pub type PeekMut < 'a , T , K , const N : usize > = PeekMutInner < 'a , T , K , OwnedVecStorage < T , N > > ;
    };
}

PeekMut!()