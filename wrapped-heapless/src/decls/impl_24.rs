macro_rules! deps {
    () => {
        Deque!();
        DequeView!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T , const N : usize > Deque < T , N > { const INIT : MaybeUninit < T > = MaybeUninit :: uninit () ; # [doc = " Constructs a new, empty deque with a fixed capacity of `N`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::Deque;"] # [doc = ""] # [doc = " // allocate the deque on the stack"] # [doc = " let mut x: Deque<u8, 16> = Deque::new();"] # [doc = ""] # [doc = " // allocate the deque in a static variable"] # [doc = " static mut X: Deque<u8, 16> = Deque::new();"] # [doc = " ```"] pub const fn new () -> Self { const { assert ! (N > 0) ; } Self { phantom : PhantomData , buffer : VecStorageInner { buffer : [Self :: INIT ; N] , } , front : 0 , back : 0 , full : false , } } # [doc = " Returns the maximum number of elements the deque can hold."] # [doc = ""] # [doc = " This method is not available on a `DequeView`, use"] # [doc = " [`storage_capacity`](DequeInner::storage_capacity) instead."] pub const fn capacity (& self) -> usize { N } # [doc = " Returns the number of elements currently in the deque."] # [doc = ""] # [doc = " This method is not available on a `DequeView`, use [`storage_len`](DequeInner::storage_len)"] # [doc = " instead."] pub const fn len (& self) -> usize { if self . full { N } else if self . back < self . front { self . back + N - self . front } else { self . back - self . front } } }
    };
}

impl_24!()