// Generated macro for unfold (function)
macro_rules! Depcrate_iterunfold {
() => {
// Module: crate::iter
// Provides: {"unfold"}
// Dependencies: {}
# [doc = " Create an iterator of values using a function to update an owned state"] # [doc = " value."] # [doc = ""] # [doc = " The function is called with the current state as its argument, and should"] # [doc = " return an [`Option`][std::option::Option] of a tuple of the next value to"] # [doc = " yield from the iterator and the updated state. If the function returns"] # [doc = " [`None`][std::option::Option::None], the iterator ends."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::iter::unfold;"] # [doc = " # use im::vector::Vector;"] # [doc = " # use std::iter::FromIterator;"] # [doc = " // Create an infinite stream of numbers, starting at 0."] # [doc = " let mut it = unfold(0, |i| Some((i, i + 1)));"] # [doc = ""] # [doc = " // Make a list out of its first five elements."] # [doc = " let numbers = Vector::from_iter(it.take(5));"] # [doc = " assert_eq!(numbers, vector![0, 1, 2, 3, 4]);"] # [doc = " ```"] # [doc = ""] # [doc = " [std::option::Option]: https://doc.rust-lang.org/std/option/enum.Option.html"] # [doc = " [std::option::Option::None]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None"] pub fn unfold < F , S , A > (value : S , f : F) -> impl Iterator < Item = A > where F : Fn (S) -> Option < (A , S) > , { let mut value = Some (value) ; std :: iter :: from_fn (move | | { f (value . take () . unwrap ()) . map (| (next , state) | { value = Some (state) ; next }) }) }
};
}
