// Generated macro for csi (macro)
macro_rules! Depcrate_macroscsi {
() => {
// Module: crate::macros
// Provides: {"csi"}
// Dependencies: {}
# [doc = " Concatenate string literals while prepending a ANSI control sequence introducer (`\"\\x1b[\"`)"] # [macro_export] # [doc (hidden)] macro_rules ! csi { ($ ($ l : expr) ,*) => { concat ! ("\x1B[" , $ ($ l) ,*) } ; }
};
}
