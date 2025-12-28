macro_rules! deps {
    () => {
        Entry!();
        Shared!();
        Owned!();
        List!();
        Guard!();
        Iter!();
        Atomic!();
        IsElement!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T , C : IsElement < T > > List < T , C > { # [doc = " Returns a new, empty linked list."] pub (crate) fn new () -> Self { Self { head : Atomic :: null () , _marker : PhantomData , } } # [doc = " Inserts `entry` into the head of the list."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You should guarantee that:"] # [doc = ""] # [doc = " - `container` is not null"] # [doc = " - `container` is immovable, e.g. inside an `Owned`"] # [doc = " - the same `Entry` is not inserted more than once"] # [doc = " - the inserted object will be removed before the list is dropped"] pub (crate) unsafe fn insert < 'g > (& 'g self , container : Shared < 'g , T > , guard : & 'g Guard) { let to = & self . head ; let entry : & Entry = C :: entry_of (unsafe { container . deref () }) ; let entry_ptr = Shared :: from (entry as * const _) ; let mut next = to . load (Relaxed , guard) ; loop { entry . next . store (next , Relaxed) ; match to . compare_exchange_weak (next , entry_ptr , Release , Relaxed , guard) { Ok (_) => break , Err (err) => next = err . current , } } } # [doc = " Returns an iterator over all objects."] # [doc = ""] # [doc = " # Caveat"] # [doc = ""] # [doc = " Every object that is inserted at the moment this function is called and persists at least"] # [doc = " until the end of iteration will be returned. Since this iterator traverses a lock-free"] # [doc = " linked list that may be concurrently modified, some additional caveats apply:"] # [doc = ""] # [doc = " 1. If a new object is inserted during iteration, it may or may not be returned."] # [doc = " 2. If an object is deleted during iteration, it may or may not be returned."] # [doc = " 3. The iteration may be aborted when it lost in a race condition. In this case, the winning"] # [doc = "    thread will continue to iterate over the same list."] pub (crate) fn iter < 'g > (& 'g self , guard : & 'g Guard) -> Iter < 'g , T , C > { Iter { guard , pred : & self . head , curr : self . head . load (Acquire , guard) , head : & self . head , _marker : PhantomData , } } }
    };
}

impl_125!()