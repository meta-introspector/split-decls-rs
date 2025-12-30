// Generated macro for osc (macro)
macro_rules! Depcrate_macrososc {
() => {
// Module: crate::macros
// Provides: {"osc"}
// Dependencies: {}
# [doc = " Concatenate string literals while prepending a xterm Operating System Commands (OSC)"] # [doc = " introducer (`\"\\x1b]\"`) and appending a BEL (`\"\\x07\"`)."] # [macro_export] # [doc (hidden)] macro_rules ! osc { ($ ($ l : expr) ,*) => { concat ! ("\x1B]" , $ ($ l) ,*, "\x1B\\") } ; }
};
}
