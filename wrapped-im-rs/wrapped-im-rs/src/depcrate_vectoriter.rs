// Generated macro for Iter (struct)
macro_rules! Depcrate_vectorIter {
() => {
// Module: crate::vector
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over vectors with values of type `A`."] # [doc = ""] # [doc = " To obtain one, use [`Vector::iter()`][iter]."] # [doc = ""] # [doc = " [iter]: enum.Vector.html#method.iter"] pub struct Iter < 'a , A > { focus : Focus < 'a , A > , front_index : usize , back_index : usize , }
};
}
