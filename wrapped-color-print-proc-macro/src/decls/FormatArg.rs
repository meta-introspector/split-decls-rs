macro_rules! FormatArg {
    () => {
        # [doc = " An argument in a `format!`-like macro (excluding the first argument aka the format string)."] pub struct FormatArg { # [doc = " The argument name in the case of a named argument, e.g. `foo` inside `foo = 1 + 1`."] pub arg_name : Option < (Ident , token :: Eq) > , # [doc = " The real argument to be formatted by the macro."] pub expr : Expr , }
    };
}

FormatArg!()