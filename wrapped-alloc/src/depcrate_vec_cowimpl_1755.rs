// Generated macro for impl_1755 (impl)
macro_rules! Depcrate_vec_cowimpl_1755 {
() => {
// Module: crate::vec::cow
// Provides: {"impl_1755"}
// Dependencies: {}
# [stable (feature = "cow_from_vec_ref" , since = "1.28.0")] impl < 'a , T : Clone > From < & 'a Vec < T > > for Cow < 'a , [T] > { # [doc = " Creates a [`Borrowed`] variant of [`Cow`]"] # [doc = " from a reference to [`Vec`]."] # [doc = ""] # [doc = " This conversion does not allocate or clone the data."] # [doc = ""] # [doc = " [`Borrowed`]: crate::borrow::Cow::Borrowed"] fn from (v : & 'a Vec < T >) -> Cow < 'a , [T] > { Cow :: Borrowed (v . as_slice ()) } }
};
}
