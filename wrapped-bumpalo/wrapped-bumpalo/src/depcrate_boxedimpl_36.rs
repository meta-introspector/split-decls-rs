// Generated macro for impl_36 (impl)
macro_rules! Depcrate_boxedimpl_36 {
() => {
// Module: crate::boxed
// Provides: {"impl_36"}
// Dependencies: {}
# [doc = " This impl replaces unsize coercion."] impl < 'a , T , const N : usize > TryFrom < Box < 'a , [T] > > for Box < 'a , [T ; N] > { type Error = Box < 'a , [T] > ; fn try_from (slice : Box < 'a , [T] >) -> Result < Box < 'a , [T ; N] > , Box < 'a , [T] > > { if slice . len () == N { let mut slice = ManuallyDrop :: new (slice) ; let ptr = slice . as_mut_ptr () as * mut [T ; N] ; Ok (unsafe { Box :: from_raw (ptr) }) } else { Err (slice) } } }
};
}
