// Generated macro for impl_1975 (impl)
macro_rules! Depcrate_vecimpl_1975 {
() => {
// Module: crate::vec
// Provides: {"impl_1975"}
// Dependencies: {}
# [stable (feature = "vec_from_cow_slice" , since = "1.14.0")] impl < 'a , T > From < Cow < 'a , [T] > > for Vec < T > where [T] : ToOwned < Owned = Vec < T > > , { # [doc = " Converts a clone-on-write slice into a vector."] # [doc = ""] # [doc = " If `s` already owns a `Vec<T>`, it will be returned directly."] # [doc = " If `s` is borrowing a slice, a new `Vec<T>` will be allocated and"] # [doc = " filled by cloning `s`'s items into it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::borrow::Cow;"] # [doc = " let o: Cow<'_, [i32]> = Cow::Owned(vec![1, 2, 3]);"] # [doc = " let b: Cow<'_, [i32]> = Cow::Borrowed(&[1, 2, 3]);"] # [doc = " assert_eq!(Vec::from(o), Vec::from(b));"] # [doc = " ```"] # [track_caller] fn from (s : Cow < 'a , [T] >) -> Vec < T > { s . into_owned () } }
};
}
