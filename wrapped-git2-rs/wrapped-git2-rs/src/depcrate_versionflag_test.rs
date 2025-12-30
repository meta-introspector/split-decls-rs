// Generated macro for flag_test (macro)
macro_rules! Depcrate_versionflag_test {
() => {
// Module: crate::version
// Provides: {"flag_test"}
// Dependencies: {}
macro_rules ! flag_test { ($ features : expr , $ flag : expr) => { ($ features as u32 & $ flag as u32) != 0 } ; }
};
}
