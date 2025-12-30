// Generated macro for impl_127 (impl)
macro_rules! Depcrate_setimpl_127 {
() => {
// Module: crate::set
// Provides: {"impl_127"}
// Dependencies: {}
impl < T > Entry < '_ , T > where T : Ord + Send + 'static , { # [doc = " Removes the entry from the set."] # [doc = ""] # [doc = " Returns `true` if this call removed the entry and `false` if it was already removed."] pub fn remove (& self) -> bool { self . inner . remove () } }
};
}
