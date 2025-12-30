// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Pipe { # [doc = " Get the length of the data in the pipe."] fn len (& self) -> usize { let head = self . head . load (Ordering :: Acquire) ; let tail = self . tail . load (Ordering :: Acquire) ; if head <= tail { tail - head } else { (2 * self . cap) - (head - tail) } } # [doc = " Given an index in `0..2*cap`, returns the real index in `0..cap`."] # [inline] fn real_index (& self , i : usize) -> usize { if i < self . cap { i } else { i - self . cap } } }
};
}
