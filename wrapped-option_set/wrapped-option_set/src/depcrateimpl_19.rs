// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Deref for Str < '_ > { type Target = str ; fn deref (& self) -> & Self :: Target { match * self { Str :: Str (s) => s , Str :: String (ref s) => s , } } }
};
}
