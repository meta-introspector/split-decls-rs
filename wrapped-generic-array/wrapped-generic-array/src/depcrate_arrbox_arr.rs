// Generated macro for box_arr (macro)
macro_rules! Depcrate_arrbox_arr {
() => {
// Module: crate::arr
// Provides: {"box_arr"}
// Dependencies: {}
# [doc = " Like [`arr!`], but returns a `Box<GenericArray<T, N>>`"] # [doc = ""] # [doc = " Unlike [`arr!`], this is not limited by stack size, only the heap."] # [doc = ""] # [doc = " Example:"] # [doc = " ```"] # [doc = " # use generic_array::{box_arr, typenum::{self, *}};"] # [doc = " // allocate a 16MB Buffer of u128 elements (16 bytes * 10 ^ 6)"] # [doc = " # #[cfg(not(miri))]"] # [doc = " let test = box_arr![1u128; typenum::Exp<U10, U6>];"] # [doc = " //  test: Box<GenericArray<u128, _>>"] # [doc = " ```"] # [doc = ""] # [doc = " # NOTES AND LIMITATIONS"] # [doc = " * The `[T; usize]` explicit and `[0, 1, 2, 3]` implicit forms are limited to lengths supported by [`Const<U>`](typenum::Const)"] # [cfg (feature = "alloc")] # [macro_export] macro_rules ! box_arr { ($ ($ x : expr) ,* $ (,) *) => ({ $ crate :: GenericArray :: __from_vec_helper ([$ ($ crate :: box_arr_helper ! (@ unit $ x)) ,*] , $ crate :: alloc :: vec ! [$ ($ x) ,*]) }) ; ($ x : expr ; $ N : ty) => ($ crate :: GenericArray ::< _ , $ N >:: try_from_vec ($ crate :: alloc :: vec ! [$ x ; <$ N as $ crate :: typenum :: Unsigned >:: USIZE]) . unwrap ()) ; ($ x : expr ; $ n : expr) => ({ const __LEN : usize = $ n ; $ crate :: GenericArray ::< _ , <$ crate :: typenum :: Const < __LEN > as $ crate :: IntoArrayLength >:: ArrayLength >:: try_from_vec ($ crate :: alloc :: vec ! [$ x ; __LEN]) . unwrap () }) ; }
};
}
