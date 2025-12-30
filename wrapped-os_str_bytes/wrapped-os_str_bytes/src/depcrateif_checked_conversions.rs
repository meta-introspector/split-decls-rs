// Generated macro for if_checked_conversions (macro)
macro_rules! Depcrateif_checked_conversions {
() => {
// Module: crate
// Provides: {"if_checked_conversions"}
// Dependencies: {}
macro_rules ! if_checked_conversions { ($ ($ item : item) +) => { $ (# [cfg (feature = "checked_conversions")] $ item) + } ; }
};
}
