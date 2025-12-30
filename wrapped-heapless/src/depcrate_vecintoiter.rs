// Generated macro for IntoIter (struct)
macro_rules! Depcrate_vecIntoIter {
() => {
// Module: crate::vec
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of an [`Vec`][`Vec`]."] # [doc = ""] # [doc = " This struct is created by calling the `into_iter` method on [`Vec`][`Vec`]."] pub struct IntoIter < T , const N : usize , LenT : LenType > { vec : Vec < T , N , LenT > , next : LenT , }
};
}
