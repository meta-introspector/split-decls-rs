// Generated macro for impl_170 (impl)
macro_rules! Depcrate_parse_fixtureimpl_170 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_170"}
// Dependencies: {}
impl FixtureData { pub (crate) fn fixtures (& self) -> impl Iterator < Item = & Fixture > { self . items . iter () . filter_map (| f | match f { FixtureItem :: Fixture (ref fixture) => Some (fixture . as_ref ()) , _ => None , }) } pub (crate) fn values (& self) -> impl Iterator < Item = & ArgumentValue > { self . items . iter () . filter_map (| f | match f { FixtureItem :: ArgumentValue (ref value) => Some (value . as_ref ()) , _ => None , }) } }
};
}
