// Generated macro for impl_1754 (impl)
macro_rules! Depcrate_vec_cowimpl_1754 {
() => {
// Module: crate::vec::cow
// Provides: {"impl_1754"}
// Dependencies: {}
# [stable (feature = "cow_from_vec" , since = "1.8.0")] impl < 'a , T : Clone > From < Vec < T > > for Cow < 'a , [T] > { # [doc = " Creates an [`Owned`] variant of [`Cow`]"] # [doc = " from an owned instance of [`Vec`]."] # [doc = ""] # [doc = " This conversion does not allocate or clone the data."] # [doc = ""] # [doc = " [`Owned`]: crate::borrow::Cow::Owned"] fn from (v : Vec < T >) -> Cow < 'a , [T] > { Cow :: Owned (v) } }
};
}
