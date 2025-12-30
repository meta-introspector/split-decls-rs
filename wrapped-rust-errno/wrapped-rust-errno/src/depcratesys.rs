// Generated macro for sys (module)
macro_rules! Depcratesys {
() => {
// Module: crate
// Provides: {"sys"}
// Dependencies: {}
# [cfg_attr (unix , path = "unix.rs")] # [cfg_attr (windows , path = "windows.rs")] # [cfg_attr (target_os = "wasi" , path = "wasi.rs")] # [cfg_attr (target_os = "hermit" , path = "hermit.rs")] mod sys ;
};
}
