// Generated macro for impl_35 (impl)
macro_rules! Depcrate_boxedimpl_35 {
() => {
// Module: crate::boxed
// Provides: {"impl_35"}
// Dependencies: {}
# [doc = " This impl replaces unsize coercion."] impl < 'a , T , const N : usize > From < Box < 'a , [T ; N] > > for Box < 'a , [T] > { fn from (arr : Box < 'a , [T ; N] >) -> Box < 'a , [T] > { let mut arr = ManuallyDrop :: new (arr) ; let ptr = core :: ptr :: slice_from_raw_parts_mut (arr . as_mut_ptr () , N) ; unsafe { Box :: from_raw (ptr) } } }
};
}
