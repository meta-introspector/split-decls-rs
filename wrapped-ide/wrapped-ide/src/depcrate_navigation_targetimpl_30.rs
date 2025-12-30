// Generated macro for impl_30 (impl)
macro_rules! Depcrate_navigation_targetimpl_30 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_30"}
// Dependencies: {}
impl UpmapFromRaFixture for NavigationTarget { fn upmap_from_ra_fixture (self , analysis : & ide_db :: ra_fixture :: RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { let virtual_file_id = self . file_id ; Ok (NavigationTarget { file_id : real_file_id , full_range : self . full_range . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id ,) ? , focus_range : self . focus_range . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id ,) ? , name : self . name . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ? , kind : self . kind . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ? , container_name : self . container_name . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id ,) ? , description : self . description . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id ,) ? , docs : self . docs . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ? , alias : self . alias . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ? , }) } }
};
}
