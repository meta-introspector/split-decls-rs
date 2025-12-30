// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_util_borrowimpl_1172 {
() => {
// Module: crate::util::borrow
// Provides: {"impl_1172"}
// Dependencies: {}
impl < 'a , T > DumbCow < 'a , T > { pub (crate) fn borrowed (& self) -> DumbCow < '_ , T > { match * self { DumbCow :: Owned (ref this) => DumbCow :: Borrowed (this) , DumbCow :: Borrowed (ref this) => DumbCow :: Borrowed (this) , } } }
};
}
