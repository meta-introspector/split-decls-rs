// Generated macro for write_seed_data_to_file (function)
macro_rules! Depcrate_test_runner_failure_persistence_filewrite_seed_data_to_file {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"write_seed_data_to_file"}
// Dependencies: {}
fn write_seed_data_to_file (dst : & Path , data : & [u8]) -> io :: Result < () > { if let Some (parent) = dst . parent () { fs :: create_dir_all (parent) ? ; } let mut options = fs :: OpenOptions :: new () ; options . append (true) . create (true) ; let mut out = options . open (dst) ? ; out . write_all (data) ? ; Ok (()) }
};
}
