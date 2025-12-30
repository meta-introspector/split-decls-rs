// Generated macro for to_idents (macro)
macro_rules! Depcrate_testto_idents {
() => {
// Module: crate::test
// Provides: {"to_idents"}
// Dependencies: {}
macro_rules ! to_idents { ($ e : expr) => { $ e . iter () . map (| s | ident (s)) . collect ::< Vec < _ >> () } ; }
};
}
