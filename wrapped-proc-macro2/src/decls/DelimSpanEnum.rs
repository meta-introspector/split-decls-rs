macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! DelimSpanEnum {
    () => {
        deps!();
        # [derive (Copy , Clone)] enum DelimSpanEnum { # [cfg (wrap_proc_macro)] Compiler { join : proc_macro :: Span , open : proc_macro :: Span , close : proc_macro :: Span , } , Fallback (fallback :: Span) , }
    };
}

DelimSpanEnum!();