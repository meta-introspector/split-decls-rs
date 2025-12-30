// Generated macro for impl_15 (impl)
macro_rules! Depcrate_connectionimpl_15 {
() => {
// Module: crate::connection
// Provides: {"impl_15"}
// Dependencies: {}
impl std :: fmt :: Display for IOResourceError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { IOResourceError :: Dbus (e) => e . fmt (f) , IOResourceError :: Io (e) => e . fmt (f) , } } }
};
}
