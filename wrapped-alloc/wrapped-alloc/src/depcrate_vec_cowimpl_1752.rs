// Generated macro for impl_1752 (impl)
macro_rules! Depcrate_vec_cowimpl_1752 {
() => {
// Module: crate::vec::cow
// Provides: {"impl_1752"}
// Dependencies: {}
# [stable (feature = "cow_from_vec" , since = "1.8.0")] impl < 'a , T : Clone > From < & 'a [T] > for Cow < 'a , [T] > { # [doc = " Creates a [`Borrowed`] variant of [`Cow`]"] # [doc = " from a slice."] # [doc = ""] # [doc = " This conversion does not allocate or clone the data."] # [doc = ""] # [doc = " [`Borrowed`]: crate::borrow::Cow::Borrowed"] fn from (s : & 'a [T]) -> Cow < 'a , [T] > { Cow :: Borrowed (s) } }
};
}
