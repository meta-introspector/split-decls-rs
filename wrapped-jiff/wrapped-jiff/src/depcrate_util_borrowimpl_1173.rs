// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_util_borrowimpl_1173 {
() => {
// Module: crate::util::borrow
// Provides: {"impl_1173"}
// Dependencies: {}
impl < 'a , T > core :: ops :: Deref for DumbCow < 'a , T > { type Target = T ; fn deref (& self) -> & T { match * self { DumbCow :: Owned (ref t) => t , DumbCow :: Borrowed (t) => t , } } }
};
}
