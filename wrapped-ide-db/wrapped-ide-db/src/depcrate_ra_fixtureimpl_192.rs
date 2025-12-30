// Generated macro for impl_192 (impl)
macro_rules! Depcrate_ra_fixtureimpl_192 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_192"}
// Dependencies: {}
impl UpmapFromRaFixture for FileRangeWrapper < FileId > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (FileRangeWrapper { file_id : real_file_id , range : self . range . upmap_from_ra_fixture (analysis , self . file_id , real_file_id) ? , }) } }
};
}
