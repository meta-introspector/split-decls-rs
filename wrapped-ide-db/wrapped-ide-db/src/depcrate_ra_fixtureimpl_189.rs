// Generated macro for impl_189 (impl)
macro_rules! Depcrate_ra_fixtureimpl_189 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_189"}
// Dependencies: {}
impl UpmapFromRaFixture for TextSize { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , _real_file_id : FileId ,) -> Result < Self , () > { analysis . map_offset_up (virtual_file_id , self) . ok_or (()) } }
};
}
