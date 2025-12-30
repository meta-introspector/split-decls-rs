// Generated macro for RaFixtureAnalysis (struct)
macro_rules! Depcrate_ra_fixtureRaFixtureAnalysis {
() => {
// Module: crate::ra_fixture
// Provides: {"RaFixtureAnalysis"}
// Dependencies: {}
pub struct RaFixtureAnalysis { pub db : RootDatabase , tmp_file_ids : Vec < (FileId , usize) > , line_offsets : Vec < TextSize > , virtual_file_id_to_line : Vec < usize > , mapper : RangeMapper , literal : ast :: String , sysroot_files : Vec < FileId > , combined_len : TextSize , }
};
}
