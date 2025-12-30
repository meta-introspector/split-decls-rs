// Generated macro for IterMut (struct)
macro_rules! Depcrate_vectorIterMut {
() => {
// Module: crate::vector
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over vectors with values of type `A`."] # [doc = ""] # [doc = " To obtain one, use [`Vector::iter_mut()`][iter_mut]."] # [doc = ""] # [doc = " [iter_mut]: enum.Vector.html#method.iter_mut"] pub struct IterMut < 'a , A > { focus : FocusMut < 'a , A > , front_index : usize , back_index : usize , }
};
}
