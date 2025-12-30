// Generated macro for impl_191 (impl)
macro_rules! Depcrate_ra_fixtureimpl_191 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_191"}
// Dependencies: {}
impl UpmapFromRaFixture for FilePositionWrapper < FileId > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (FilePositionWrapper { file_id : real_file_id , offset : self . offset . upmap_from_ra_fixture (analysis , self . file_id , real_file_id) ? , }) } }
};
}
