macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! _SyntaxElement {
    () => {
        deps!();
        struct _SyntaxElement ;
    };
}

_SyntaxElement!()