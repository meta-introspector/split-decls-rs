// Generated macro for impl_648 (impl)
macro_rules! Depcrate_map_extimpl_648 {
() => {
// Module: crate::map_ext
// Provides: {"impl_648"}
// Dependencies: {}
impl HeaderMapExt for http :: HeaderMap { fn typed_insert < H > (& mut self , header : H) where H : Header , { let entry = self . entry (H :: name ()) ; let mut values = ToValues { state : State :: First (entry) , } ; header . encode (& mut values) ; } fn typed_get < H > (& self) -> Option < H > where H : Header , { HeaderMapExt :: typed_try_get (self) . unwrap_or (None) } fn typed_try_get < H > (& self) -> Result < Option < H > , Error > where H : Header , { let mut values = self . get_all (H :: name ()) . iter () ; if values . size_hint () == (0 , Some (0)) { Ok (None) } else { H :: decode (& mut values) . map (Some) } } }
};
}
