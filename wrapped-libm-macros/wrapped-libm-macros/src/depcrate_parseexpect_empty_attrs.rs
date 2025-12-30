// Generated macro for expect_empty_attrs (function)
macro_rules! Depcrate_parseexpect_empty_attrs {
() => {
// Module: crate::parse
// Provides: {"expect_empty_attrs"}
// Dependencies: {}
fn expect_empty_attrs (attrs : & [Attribute]) -> syn :: Result < () > { if attrs . is_empty () { return Ok (()) ; } let e = syn :: Error :: new (attrs . first () . unwrap () . span () , "no attributes allowed in this position" ,) ; Err (e) }
};
}
