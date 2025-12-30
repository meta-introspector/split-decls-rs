// Generated macro for KeyFromPath (trait)
macro_rules! Depcrate_from_metaKeyFromPath {
() => {
// Module: crate::from_meta
// Provides: {"KeyFromPath"}
// Dependencies: {}
# [doc = " Trait to convert from a path into an owned key for a map."] trait KeyFromPath : Sized { fn from_path (path : & syn :: Path) -> Result < Self > ; fn to_display (& self) -> Cow < '_ , str > ; }
};
}
