// Generated macro for DOCKER_DIRECTORY (const)
macro_rules! DepcrateDOCKER_DIRECTORY {
() => {
// Module: crate
// Provides: {"DOCKER_DIRECTORY"}
// Dependencies: {}
pub const DOCKER_DIRECTORY : & str = concat ! (env ! ("CARGO_MANIFEST_DIR") , "/../docker") ;
};
}
