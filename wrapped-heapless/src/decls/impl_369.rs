macro_rules! deps {
    () => {
        BinaryHeapView!();
        BinaryHeap!();
        BinaryHeapInner!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < T , K , S : VecStorage < T > > BinaryHeapInner < T , K , S > { # [doc = " Get a reference to the `BinaryHeap`, erasing the `N` const-generic."] pub fn as_view (& self) -> & BinaryHeapView < T , K > { S :: as_binary_heap_view (self) } # [doc = " Get a mutable reference to the `BinaryHeap`, erasing the `N` const-generic."] pub fn as_mut_view (& mut self) -> & mut BinaryHeapView < T , K > { S :: as_binary_heap_view_mut (self) } }
    };
}

impl_369!()