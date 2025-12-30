// Generated macro for CI_DIRECTORY (const)
macro_rules! DepcrateCI_DIRECTORY {
() => {
// Module: crate
// Provides: {"CI_DIRECTORY"}
// Dependencies: {}
const CI_DIRECTORY : & str = concat ! (env ! ("CARGO_MANIFEST_DIR") , "/..") ;
};
}
