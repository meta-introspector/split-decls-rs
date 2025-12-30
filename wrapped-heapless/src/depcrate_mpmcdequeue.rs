// Generated macro for dequeue (function)
macro_rules! Depcrate_mpmcdequeue {
() => {
// Module: crate::mpmc
// Provides: {"dequeue"}
// Dependencies: {}
unsafe fn dequeue < T > (buffer : * mut Cell < T > , dequeue_pos : & AtomicTargetSize , mask : UintSize ,) -> Option < T > { let mut pos = dequeue_pos . load (Ordering :: Relaxed) ; let mut cell ; loop { cell = buffer . add (usize :: from (pos & mask)) ; let seq = (* cell) . sequence . load (Ordering :: Acquire) ; let dif = (seq as IntSize) . wrapping_sub ((pos . wrapping_add (1)) as IntSize) ; match dif . cmp (& 0) { core :: cmp :: Ordering :: Equal => { if dequeue_pos . compare_exchange_weak (pos , pos . wrapping_add (1) , Ordering :: Relaxed , Ordering :: Relaxed ,) . is_ok () { break ; } } core :: cmp :: Ordering :: Less => { return None ; } core :: cmp :: Ordering :: Greater => { pos = dequeue_pos . load (Ordering :: Relaxed) ; } } } let data = (* cell) . data . as_ptr () . read () ; (* cell) . sequence . store (pos . wrapping_add (mask) . wrapping_add (1) , Ordering :: Release) ; Some (data) }
};
}
