// Generated macro for impl_466 (impl)
macro_rules! Depcrate_from_metaimpl_466 {
() => {
// Module: crate::from_meta
// Provides: {"impl_466"}
// Dependencies: {}
impl KeyFromPath for String { fn from_path (path : & syn :: Path) -> Result < Self > { Ok (path_to_string (path)) } fn to_display (& self) -> Cow < '_ , str > { Cow :: Borrowed (self) } }
};
}
