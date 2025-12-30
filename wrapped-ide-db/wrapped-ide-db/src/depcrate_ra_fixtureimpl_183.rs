// Generated macro for impl_183 (impl)
macro_rules! Depcrate_ra_fixtureimpl_183 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_183"}
// Dependencies: {}
impl < T : UpmapFromRaFixture > UpmapFromRaFixture for Vec < T > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { upmap_collection (self , analysis , virtual_file_id , real_file_id) } }
};
}
