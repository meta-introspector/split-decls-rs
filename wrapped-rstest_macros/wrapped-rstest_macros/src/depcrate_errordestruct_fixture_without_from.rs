// Generated macro for destruct_fixture_without_from (function)
macro_rules! Depcrate_errordestruct_fixture_without_from {
() => {
// Module: crate::error
// Provides: {"destruct_fixture_without_from"}
// Dependencies: {}
fn destruct_fixture_without_from < 'a > (function : & 'a ItemFn , info : & 'a impl IsImplicitFixture ,) -> Errors < 'a > { Box :: new (function . sig . inputs . iter () . filter_map (| a | a . maybe_pat () . map (| p | (a , p))) . filter (| & (_ , p) | p . maybe_ident () . is_none ()) . filter (| & (_ , p) | info . is_implicit_fixture (p)) . map (| (a , _) | syn :: Error :: new_spanned (a , messages :: DESTRUCT_WITHOUT_FROM)) ,) }
};
}
