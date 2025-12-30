// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1122 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1122"}
// Dependencies: {}
# [stable (feature = "box_from_cow" , since = "1.45.0")] impl From < Cow < '_ , CStr > > for Box < CStr > { # [doc = " Converts a `Cow<'a, CStr>` into a `Box<CStr>`,"] # [doc = " by copying the contents if they are borrowed."] # [inline] fn from (cow : Cow < '_ , CStr >) -> Box < CStr > { match cow { Cow :: Borrowed (s) => Box :: from (s) , Cow :: Owned (s) => Box :: from (s) , } } }
};
}
