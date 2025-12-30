// Generated macro for emit_delayed_lint (function)
macro_rules! Depcrateemit_delayed_lint {
() => {
// Module: crate
// Provides: {"emit_delayed_lint"}
// Dependencies: {}
fn emit_delayed_lint (lint : & DelayedLint , tcx : TyCtxt < '_ >) { match lint { DelayedLint :: AttributeParsing (attribute_lint) => { rustc_attr_parsing :: emit_attribute_lint (attribute_lint , tcx) } } }
};
}
