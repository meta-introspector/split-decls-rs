// Generated macro for to_strs (macro)
macro_rules! Depcrate_testto_strs {
() => {
// Module: crate::test
// Provides: {"to_strs"}
// Dependencies: {}
macro_rules ! to_strs { ($ e : expr) => { $ e . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () } ; }
};
}
