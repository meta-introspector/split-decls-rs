// Generated macro for impl_1753 (impl)
macro_rules! Depcrate_vec_cowimpl_1753 {
() => {
// Module: crate::vec::cow
// Provides: {"impl_1753"}
// Dependencies: {}
# [stable (feature = "cow_from_array_ref" , since = "1.77.0")] impl < 'a , T : Clone , const N : usize > From < & 'a [T ; N] > for Cow < 'a , [T] > { # [doc = " Creates a [`Borrowed`] variant of [`Cow`]"] # [doc = " from a reference to an array."] # [doc = ""] # [doc = " This conversion does not allocate or clone the data."] # [doc = ""] # [doc = " [`Borrowed`]: crate::borrow::Cow::Borrowed"] fn from (s : & 'a [T ; N]) -> Cow < 'a , [T] > { Cow :: Borrowed (s as & [_]) } }
};
}
