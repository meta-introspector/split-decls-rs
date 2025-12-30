// Generated macro for move_file (function)
macro_rules! Depcrate_utils_helpersmove_file {
() => {
// Module: crate::utils::helpers
// Provides: {"move_file"}
// Dependencies: {}
# [doc = " Rename a file if from and to are in the same filesystem or"] # [doc = " copy and remove the file otherwise"] pub fn move_file < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) -> io :: Result < () > { match fs :: rename (& from , & to) { Err (e) if e . kind () == io :: ErrorKind :: CrossesDevices => { std :: fs :: copy (& from , & to) ? ; std :: fs :: remove_file (& from) } r => r , } }
};
}
