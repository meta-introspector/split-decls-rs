// Generated macro for custom_prefix (module)
macro_rules! Depcratecustom_prefix {
() => {
// Module: crate
// Provides: {"custom_prefix"}
// Dependencies: {}
# [cfg (feature = "gz")] # [allow (unused)] mod custom_prefix { # [cfg (feature = "custom-prefix")] macro_rules ! prefix { ($ name : expr) => { concat ! (env ! ("LIBZ_RS_SYS_PREFIX") , stringify ! ($ name)) } ; } const _PRE_ONE_DOT_O : () = assert ! (env ! ("CARGO_PKG_VERSION_MAJOR") . as_bytes () [0] == b'0') ; # [cfg (feature = "semver-prefix")] macro_rules ! prefix { ($ name : expr) => { concat ! ("LIBZ_RS_SYS_v" , env ! ("CARGO_PKG_VERSION_MAJOR") , "_" , env ! ("CARGO_PKG_VERSION_MINOR") , "_x_" , stringify ! ($ name)) } ; } # [cfg (all (not (feature = "custom-prefix") , not (feature = "semver-prefix") ,))] macro_rules ! prefix { ($ name : expr) => { stringify ! ($ name) } ; } # [cfg (all (not (feature = "custom-prefix") , not (feature = "semver-prefix") , test))] macro_rules ! prefix { ($ name : expr) => { concat ! ("LIBZ_RS_SYS_TEST_" , stringify ! ($ name)) } ; } pub (crate) use prefix ; }
};
}
