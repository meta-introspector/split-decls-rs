macro_rules! deps {
    () => {
        PeekMutInner!();
        BinaryHeap!();
    };
}

macro_rules! PeekMutView {
    () => {
        deps!();
        # [doc = " Structure wrapping a mutable reference to the greatest item on a"] # [doc = " `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeapView::peek_mut`]."] # [doc = " See its documentation for more."] pub type PeekMutView < 'a , T , K > = PeekMutInner < 'a , T , K , ViewVecStorage < T > > ;
    };
}

PeekMutView!()