// Generated macro for impl_243 (impl)
macro_rules! Depcrate_valueimpl_243 {
() => {
// Module: crate::value
// Provides: {"impl_243"}
// Dependencies: {}
impl Value { fn major_type (& self) -> u8 { use self :: Value :: * ; match self { Null => 7 , Bool (_) => 7 , Integer (v) => { if * v >= 0 { 0 } else { 1 } } Tag (_ , _) => 6 , Float (_) => 7 , Bytes (_) => 2 , Text (_) => 3 , Array (_) => 4 , Map (_) => 5 , __Hidden => unreachable ! () , } } }
};
}
