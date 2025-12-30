// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl std :: fmt :: Debug for ParamsPrinter < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . 0 . iter () . copied ()) . finish () } }
};
}
