// Generated macro for vec_into_parts (function)
macro_rules! Depcratevec_into_parts {
() => {
// Module: crate
// Provides: {"vec_into_parts"}
// Dependencies: {}
fn vec_into_parts < T > (vec : Vec < T >) -> (NonNull < T > , usize , usize) { let mut vec = ManuallyDrop :: new (vec) ; (unsafe { NonNull :: new_unchecked (vec . as_mut_ptr ()) } , vec . capacity () , vec . len () ,) }
};
}
