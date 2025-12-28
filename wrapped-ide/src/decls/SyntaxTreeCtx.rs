macro_rules! deps {
    () => {
        InStringCtx!();
    };
}

macro_rules! SyntaxTreeCtx {
    () => {
        deps!();
        struct SyntaxTreeCtx { line_index : Arc < LineIndex > , in_string : Option < InStringCtx > , }
    };
}

SyntaxTreeCtx!();