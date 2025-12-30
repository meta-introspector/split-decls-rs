// Generated macro for sgr (macro)
macro_rules! Depcrate_macrossgr {
() => {
// Module: crate::macros
// Provides: {"sgr"}
// Dependencies: {}
# [doc = " Creates a select graphic rendition sequence."] # [doc = ""] # [doc = " This macro prepends provided sequence with the `ESC[` (`\\x1B[`) character and appends `m` character."] # [doc = ""] # [doc = " Also known as Set Graphics Rendition on Linux."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use anes::sgr;"] # [doc = ""] # [doc = " assert_eq!(sgr!(\"0\"), \"\\x1B[0m\");"] # [doc = " ```"] # [macro_export] macro_rules ! sgr { ($ ($ arg : expr_2021) ,*) => { concat ! ("\x1B[" , $ ($ arg) ,* , "m") } ; }
};
}
