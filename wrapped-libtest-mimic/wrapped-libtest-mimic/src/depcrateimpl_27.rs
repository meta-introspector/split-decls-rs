// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl TestInfo { fn test_name_with_kind (& self) -> Cow < '_ , str > { if self . kind . is_empty () { Cow :: Borrowed (& self . name) } else { Cow :: Owned (format ! ("[{}] {}" , self . kind , self . name)) } } }
};
}
