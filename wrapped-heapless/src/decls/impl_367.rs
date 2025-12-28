macro_rules! deps {
    () => {
        Max!();
        Vec!();
        BinaryHeap!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < T , K , const N : usize > BinaryHeap < T , K , N > { # [doc = " Creates an empty `BinaryHeap` as a $K-heap."] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::binary_heap::{BinaryHeap, Max};"] # [doc = ""] # [doc = " // allocate the binary heap on the stack"] # [doc = " let mut heap: BinaryHeap<_, Max, 8> = BinaryHeap::new();"] # [doc = " heap.push(4).unwrap();"] # [doc = ""] # [doc = " // allocate the binary heap in a static variable"] # [doc = " static mut HEAP: BinaryHeap<i32, Max, 8> = BinaryHeap::new();"] # [doc = " ```"] pub const fn new () -> Self { Self { _kind : PhantomData , data : Vec :: new () , } } }
    };
}

impl_367!()