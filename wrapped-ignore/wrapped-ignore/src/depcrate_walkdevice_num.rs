// Generated macro for device_num (function)
macro_rules! Depcrate_walkdevice_num {
() => {
// Module: crate::walk
// Provides: {"device_num"}
// Dependencies: {}
# [cfg (not (any (unix , windows)))] fn device_num < P : AsRef < Path > > (_ : P) -> io :: Result < u64 > { Err (io :: Error :: new (io :: ErrorKind :: Other , "walkdir: same_file_system option not supported on this platform" ,)) }
};
}
