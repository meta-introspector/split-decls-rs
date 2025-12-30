// Generated macro for to_pats (macro)
macro_rules! Depcrate_testto_pats {
() => {
// Module: crate::test
// Provides: {"to_pats"}
// Dependencies: {}
macro_rules ! to_pats { ($ e : expr) => { $ e . iter () . map (| s | pat (s)) . collect ::< Vec < _ >> () } ; }
};
}
