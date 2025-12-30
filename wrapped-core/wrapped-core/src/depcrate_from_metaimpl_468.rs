// Generated macro for impl_468 (impl)
macro_rules! Depcrate_from_metaimpl_468 {
() => {
// Module: crate::from_meta
// Provides: {"impl_468"}
// Dependencies: {}
impl KeyFromPath for syn :: Ident { fn from_path (path : & syn :: Path) -> Result < Self > { if path . segments . len () == 1 && path . leading_colon . is_none () && path . segments [0] . arguments . is_empty () { Ok (path . segments [0] . ident . clone ()) } else { Err (Error :: custom ("Key must be an identifier") . with_span (path)) } } fn to_display (& self) -> Cow < '_ , str > { Cow :: Owned (self . to_string ()) } }
};
}
