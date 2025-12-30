// Generated macro for PeekMut (struct)
macro_rules! Depcrate_vec_peek_mutPeekMut {
() => {
// Module: crate::vec::peek_mut
// Provides: {"PeekMut"}
// Dependencies: {}
# [doc = " Structure wrapping a mutable reference to the last item in a"] # [doc = " `Vec`."] # [doc = ""] # [doc = " This `struct` is created by the [`peek_mut`] method on [`Vec`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`peek_mut`]: Vec::peek_mut"] # [unstable (feature = "vec_peek_mut" , issue = "122742")] pub struct PeekMut < 'a , T > { vec : & 'a mut Vec < T > , }
};
}
