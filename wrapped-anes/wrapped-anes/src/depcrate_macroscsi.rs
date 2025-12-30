// Generated macro for csi (macro)
macro_rules! Depcrate_macroscsi {
() => {
// Module: crate::macros
// Provides: {"csi"}
// Dependencies: {}
# [doc = " Creates a control sequence."] # [doc = ""] # [doc = " This macro prepends provided sequence with the control sequence introducer `ESC [` (`\\x1B[`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use anes::csi;"] # [doc = ""] # [doc = " assert_eq!(csi!(\"?1049h\"), \"\\x1B[?1049h\");"] # [doc = " ```"] # [macro_export] macro_rules ! csi { ($ ($ arg : expr_2021) ,*) => { concat ! ("\x1B[" , $ ($ arg) ,*) } ; }
};
}
