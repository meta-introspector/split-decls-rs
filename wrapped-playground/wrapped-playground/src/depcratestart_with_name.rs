// Generated macro for start_with_name (function)
macro_rules! Depcratestart_with_name {
() => {
// Module: crate
// Provides: {"start_with_name"}
// Dependencies: {}
# [rstest] fn start_with_name (# [files ("files/*.txt")] path : PathBuf) { let name = path . file_name () . unwrap () ; let mut f = File :: open (& path) . unwrap () ; let mut contents = String :: new () ; f . read_to_string (& mut contents) . unwrap () ; assert ! (contents . starts_with (name . to_str () . unwrap ())) }
};
}
