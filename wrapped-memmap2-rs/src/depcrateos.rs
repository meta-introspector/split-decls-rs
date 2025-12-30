// Generated macro for os (module)
macro_rules! Depcrateos {
() => {
// Module: crate
// Provides: {"os"}
// Dependencies: {}
# [cfg_attr (unix , path = "unix.rs")] # [cfg_attr (windows , path = "windows.rs")] # [cfg_attr (not (any (unix , windows)) , path = "stub.rs")] mod os ;
};
}
