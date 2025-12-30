// Generated macro for to_exprs (macro)
macro_rules! Depcrate_testto_exprs {
() => {
// Module: crate::test
// Provides: {"to_exprs"}
// Dependencies: {}
macro_rules ! to_exprs { ($ e : expr) => { $ e . iter () . map (| s | expr (s)) . collect ::< Vec < _ >> () } ; }
};
}
