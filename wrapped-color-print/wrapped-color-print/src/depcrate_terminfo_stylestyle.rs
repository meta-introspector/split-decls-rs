// Generated macro for style (function)
macro_rules! Depcrate_terminfo_stylestyle {
() => {
// Module: crate::terminfo::style
// Provides: {"style"}
// Dependencies: {}
# [doc = " Gets the ANSI code which sets the given style `T`."] fn style < 'a , T > () -> String where T : Capability < 'a > + AsRef < [u8] > , { expand0 :: < 'a , T > () . unwrap_or_else (| | String :: new ()) }
};
}
