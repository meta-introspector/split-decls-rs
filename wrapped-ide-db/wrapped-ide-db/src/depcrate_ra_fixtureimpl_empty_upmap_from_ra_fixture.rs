// Generated macro for impl_empty_upmap_from_ra_fixture (macro)
macro_rules! Depcrate_ra_fixtureimpl_empty_upmap_from_ra_fixture {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_empty_upmap_from_ra_fixture"}
// Dependencies: {}
# [macro_export] macro_rules ! impl_empty_upmap_from_ra_fixture { ($ ($ ty : ty) ,* $ (,) ?) => { $ (impl $ crate :: ra_fixture :: UpmapFromRaFixture for $ ty { fn upmap_from_ra_fixture (self , _analysis : &$ crate :: ra_fixture :: RaFixtureAnalysis , _virtual_file_id : $ crate :: ra_fixture :: FileId , _real_file_id : $ crate :: ra_fixture :: FileId ,) -> Result < Self , () > { Ok (self) } }) * } ; }
};
}
