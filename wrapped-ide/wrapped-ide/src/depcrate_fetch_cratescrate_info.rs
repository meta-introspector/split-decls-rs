// Generated macro for crate_info (function)
macro_rules! Depcrate_fetch_cratescrate_info {
() => {
// Module: crate::fetch_crates
// Provides: {"crate_info"}
// Dependencies: {}
fn crate_info (data : & ide_db :: base_db :: BuiltCrateData , extra_data : & ide_db :: base_db :: ExtraCrateData ,) -> CrateInfo { let crate_name = crate_name (extra_data) ; let version = extra_data . version . clone () ; CrateInfo { name : crate_name , version , root_file_id : data . root_file_id } }
};
}
