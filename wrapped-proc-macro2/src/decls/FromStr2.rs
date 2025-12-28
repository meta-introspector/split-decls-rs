macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! FromStr2 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] pub (crate) trait FromStr2 : FromStr < Err = proc_macro :: LexError > { # [cfg (wrap_proc_macro)] fn valid (src : & str) -> bool ; # [cfg (wrap_proc_macro)] fn from_str_checked (src : & str) -> Result < Self , imp :: LexError > { if ! Self :: valid (src) { return Err (imp :: LexError :: CompilerPanic) ; } match panic :: catch_unwind (| | Self :: from_str (src)) { Ok (Ok (ok)) => Ok (ok) , Ok (Err (lex)) => Err (imp :: LexError :: Compiler (lex)) , Err (_panic) => Err (imp :: LexError :: CompilerPanic) , } } fn from_str_unchecked (src : & str) -> Self { Self :: from_str (src) . unwrap () } }
    };
}

FromStr2!()