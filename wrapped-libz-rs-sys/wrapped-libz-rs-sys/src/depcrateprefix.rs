// Generated macro for prefix (macro)
macro_rules! Depcrateprefix {
() => {
// Module: crate
// Provides: {"prefix"}
// Dependencies: {}
# [cfg (all (not (feature = "custom-prefix") , not (feature = "semver-prefix") , any (test , feature = "testing-prefix")))] macro_rules ! prefix { ($ name : expr) => { concat ! ("LIBZ_RS_SYS_TEST_" , stringify ! ($ name)) } ; }
};
}
