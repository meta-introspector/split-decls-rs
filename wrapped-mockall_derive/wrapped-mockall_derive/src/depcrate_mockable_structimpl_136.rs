// Generated macro for impl_136 (impl)
macro_rules! Depcrate_mockable_structimpl_136 {
() => {
// Module: crate::mockable_struct
// Provides: {"impl_136"}
// Dependencies: {}
impl MockableStruct { # [doc = " Does this struct derive Debug?"] pub fn derives_debug (& self) -> bool { self . attrs . iter () . any (| attr | { let mut derive_debug = false ; if attr . path () . is_ident ("derive") { attr . parse_nested_meta (| meta | { if meta . path . is_ident ("Debug") { derive_debug = true ; } Ok (()) }) . unwrap () ; } derive_debug }) } }
};
}
