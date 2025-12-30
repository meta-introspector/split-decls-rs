// Generated macro for impl_16 (impl)
macro_rules! Depcrate_connectionimpl_16 {
() => {
// Module: crate::connection
// Provides: {"impl_16"}
// Dependencies: {}
impl std :: error :: Error for IOResourceError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { Some (match self { IOResourceError :: Dbus (e) => e , IOResourceError :: Io (e) => e , }) } }
};
}
