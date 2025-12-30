// Generated macro for impl_154 (impl)
macro_rules! Depcrate_read_utilimpl_154 {
() => {
// Module: crate::read::util
// Provides: {"impl_154"}
// Dependencies: {}
# [cfg (feature = "read")] unsafe impl < T > Sealed for Vec < T > { type Storage = Box < [MaybeUninit < T >] > ; fn new_storage () -> Self :: Storage { Box :: new ([]) } fn grow (storage : & mut Self :: Storage , additional : usize) -> Result < () , CapacityFull > { let mut vec : Vec < _ > = core :: mem :: replace (storage , Box :: new ([])) . into () ; vec . reserve (additional) ; unsafe { vec . set_len (vec . capacity ()) } ; * storage = vec . into_boxed_slice () ; Ok (()) } }
};
}
