// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for RefMut < '_ , T > { fn deref_mut (& mut self) -> & mut < Self as core :: ops :: Deref > :: Target { unsafe { self . value . as_mut () } } }
};
}
