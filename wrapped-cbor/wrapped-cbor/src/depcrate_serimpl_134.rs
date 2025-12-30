// Generated macro for impl_134 (impl)
macro_rules! Depcrate_serimpl_134 {
() => {
// Module: crate::ser
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , W > CollectionSerializer < 'a , W > where W : Write , { # [inline] fn end_inner (self) -> Result < () > { if self . needs_eof { self . ser . writer . write_all (& [0xff]) . map_err (| e | e . into ()) } else { Ok (()) } } }
};
}
