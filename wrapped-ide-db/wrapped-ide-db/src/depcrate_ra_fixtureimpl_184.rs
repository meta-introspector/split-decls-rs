// Generated macro for impl_184 (impl)
macro_rules! Depcrate_ra_fixtureimpl_184 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_184"}
// Dependencies: {}
impl < T : UpmapFromRaFixture , const N : usize > UpmapFromRaFixture for SmallVec < [T ; N] > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { upmap_collection (self , analysis , virtual_file_id , real_file_id) } }
};
}
