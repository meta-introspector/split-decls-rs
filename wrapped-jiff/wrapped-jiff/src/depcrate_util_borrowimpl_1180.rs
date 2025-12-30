// Generated macro for impl_1180 (impl)
macro_rules! Depcrate_util_borrowimpl_1180 {
() => {
// Module: crate::util::borrow
// Provides: {"impl_1180"}
// Dependencies: {}
impl < 'a > From < & 'a str > for StringCow < 'a > { fn from (string : & 'a str) -> StringCow < 'a > { StringCow :: Borrowed (string) } }
};
}
