// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Ord for JniVersion { fn cmp (& self , other : & Self) -> Ordering { match self . major . cmp (& other . major) { Ordering :: Equal => self . minor . cmp (& other . minor) , major_order => major_order , } } }
};
}
