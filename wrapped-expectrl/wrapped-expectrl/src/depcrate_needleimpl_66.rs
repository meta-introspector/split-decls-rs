// Generated macro for impl_66 (impl)
macro_rules! Depcrate_needleimpl_66 {
() => {
// Module: crate::needle
// Provides: {"impl_66"}
// Dependencies: {}
impl < T > Needle for Any < & [T] > where T : Needle , { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { for needle in self . 0 . iter () { let found = needle . check (buf , eof) ? ; if ! found . is_empty () { return Ok (found) ; } } Ok (Vec :: new ()) } }
};
}
