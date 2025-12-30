// Generated macro for abort (macro)
macro_rules! Depcrate_macrosabort {
() => {
// Module: crate::macros
// Provides: {"abort"}
// Dependencies: {}
# [doc = " Abort proc-macro execution right now and display the error."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [macro_export] macro_rules ! abort { ($ err : expr) => { $ crate :: diagnostic ! ($ err) . abort () } ; ($ span : expr , $ ($ tts : tt) *) => { $ crate :: diagnostic ! ($ span , $ crate :: Level :: Error , $ ($ tts) *) . abort () } ; }
};
}
