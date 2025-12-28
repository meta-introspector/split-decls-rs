macro_rules! deps {
    () => {
        IntSize!();
        AtomicTargetSize!();
        UintSize!();
        Cell!();
    };
}

macro_rules! enqueue {
    () => {
        deps!();
        unsafe fn enqueue < T > (buffer : * mut Cell < T > , enqueue_pos : & AtomicTargetSize , mask : UintSize , item : T ,) -> Result < () , T > { let mut pos = enqueue_pos . load (Ordering :: Relaxed) ; let mut cell ; loop { cell = buffer . add (usize :: from (pos & mask)) ; let seq = (* cell) . sequence . load (Ordering :: Acquire) ; let dif = (seq as IntSize) . wrapping_sub (pos as IntSize) ; match dif . cmp (& 0) { core :: cmp :: Ordering :: Equal => { if enqueue_pos . compare_exchange_weak (pos , pos . wrapping_add (1) , Ordering :: Relaxed , Ordering :: Relaxed ,) . is_ok () { break ; } } core :: cmp :: Ordering :: Less => { return Err (item) ; } core :: cmp :: Ordering :: Greater => { pos = enqueue_pos . load (Ordering :: Relaxed) ; } } } (* cell) . data . as_mut_ptr () . write (item) ; (* cell) . sequence . store (pos . wrapping_add (1) , Ordering :: Release) ; Ok (()) }
    };
}

enqueue!();