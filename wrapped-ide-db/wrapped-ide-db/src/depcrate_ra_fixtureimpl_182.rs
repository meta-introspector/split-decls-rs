// Generated macro for impl_182 (impl)
macro_rules! Depcrate_ra_fixtureimpl_182 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_182"}
// Dependencies: {}
impl < T : UpmapFromRaFixture > UpmapFromRaFixture for Option < T > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (match self { Some (it) => Some (it . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ?) , None => None , }) } }
};
}
