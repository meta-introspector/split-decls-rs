// Generated macro for esc (macro)
macro_rules! Depcrate_macrosesc {
() => {
// Module: crate::macros
// Provides: {"esc"}
// Dependencies: {}
# [doc = " Creates an escape sequence."] # [doc = ""] # [doc = " This macro prepends provided sequence with the `ESC` (`\\x1B`) character."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use anes::esc;"] # [doc = ""] # [doc = " assert_eq!(esc!(\"7\"), \"\\x1B7\");"] # [doc = " ```"] # [macro_export] macro_rules ! esc { ($ ($ arg : expr_2021) ,*) => { concat ! ("\x1B" , $ ($ arg) ,*) } ; }
};
}
