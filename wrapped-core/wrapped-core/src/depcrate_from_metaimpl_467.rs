// Generated macro for impl_467 (impl)
macro_rules! Depcrate_from_metaimpl_467 {
() => {
// Module: crate::from_meta
// Provides: {"impl_467"}
// Dependencies: {}
impl KeyFromPath for syn :: Path { fn from_path (path : & syn :: Path) -> Result < Self > { Ok (path . clone ()) } fn to_display (& self) -> Cow < '_ , str > { Cow :: Owned (path_to_string (self)) } }
};
}
