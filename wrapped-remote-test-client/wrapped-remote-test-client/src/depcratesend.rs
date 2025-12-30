// Generated macro for send (function)
macro_rules! Depcratesend {
() => {
// Module: crate
// Provides: {"send"}
// Dependencies: {}
fn send (path : & Path , dst : & mut dyn Write) { t ! (dst . write_all (path . file_name () . unwrap () . to_str () . unwrap () . as_bytes ())) ; t ! (dst . write_all (& [0])) ; let mut file = t ! (File :: open (& path)) ; let amt = t ! (file . metadata ()) . len () ; t ! (dst . write_all (& amt . to_be_bytes ())) ; t ! (io :: copy (& mut file , dst)) ; }
};
}
