// Generated macro for to_fnargs (macro)
macro_rules! Depcrate_testto_fnargs {
() => {
// Module: crate::test
// Provides: {"to_fnargs"}
// Dependencies: {}
macro_rules ! to_fnargs { ($ e : expr) => { { $ e . iter () . map (fn_arg) . collect ::< Vec < _ >> () } } ; }
};
}
