// Generated macro for impl_190 (impl)
macro_rules! Depcrate_ra_fixtureimpl_190 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_190"}
// Dependencies: {}
impl UpmapFromRaFixture for TextRange { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , _real_file_id : FileId ,) -> Result < Self , () > { analysis . map_range_up (virtual_file_id , self) . next () . ok_or (()) } }
};
}
