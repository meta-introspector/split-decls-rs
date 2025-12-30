// Generated macro for if_conversions (macro)
macro_rules! Depcrateif_conversions {
() => {
// Module: crate
// Provides: {"if_conversions"}
// Dependencies: {}
macro_rules ! if_conversions { ($ ($ item : item) +) => { $ (# [cfg (feature = "conversions")] $ item) + } ; }
};
}
