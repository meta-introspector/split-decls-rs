// Generated macro for fixture_mod (macro)
macro_rules! Depcrate_hpack_test_fixturefixture_mod {
() => {
// Module: crate::hpack::test::fixture
// Provides: {"fixture_mod"}
// Dependencies: {}
macro_rules ! fixture_mod { ($ module : ident => { $ (($ fn : ident , $ path : expr) ;) + }) => { mod $ module { $ (# [test] fn $ fn () { let path = :: std :: path :: Path :: new (env ! ("CARGO_MANIFEST_DIR")) . join ("fixtures/hpack") . join ($ path) ; super :: test_fixture (path . as_ref ()) ; }) + } } }
};
}
