// Generated macro for NCR_EXTRA (const)
macro_rules! DepcrateNCR_EXTRA {
() => {
// Module: crate
// Provides: {"NCR_EXTRA"}
// Dependencies: {}
# [doc = " This has to be the max length of an NCR instead of max"] # [doc = " minus one, because we can't rely on getting the minus"] # [doc = " one from the space reserved for the current unmappable,"] # [doc = " because the ISO-2022-JP encoder can fill up that space"] # [doc = " with a state transition escape."] const NCR_EXTRA : usize = 10 ;
};
}
