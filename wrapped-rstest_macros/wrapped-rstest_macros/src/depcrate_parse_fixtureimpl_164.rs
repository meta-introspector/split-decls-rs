// Generated macro for impl_164 (impl)
macro_rules! Depcrate_parse_fixtureimpl_164 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_164"}
// Dependencies: {}
impl Parse for FixtureInfo { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (if input . is_empty () { Default :: default () } else { Self { data : input . parse () ? , attributes : input . parse :: < Token ! [::] > () . or_else (| _ | Ok (Default :: default ())) . and_then (| _ | input . parse ()) ? , arguments : Default :: default () , } }) } }
};
}
