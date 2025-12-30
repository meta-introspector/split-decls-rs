// Generated macro for if_raw_str (macro)
macro_rules! Depcrateif_raw_str {
() => {
// Module: crate
// Provides: {"if_raw_str"}
// Dependencies: {}
macro_rules ! if_raw_str { ($ ($ item : item) +) => { $ (# [cfg (feature = "raw_os_str")] $ item) + } ; }
};
}
