macro_rules! deps {
    () => {
        SyntaxNodePtr!();
        AstNode!();
        Language!();
    };
}

macro_rules! AstPtr {
    () => {
        deps!();
        # [doc = " Like [`SyntaxNodePtr`], but remembers the type of node."] # [doc = ""] # [doc = " ## Note"] # [doc = " As with [`SyntaxNodePtr`], this must not be used on mutable"] # [doc = " syntax trees, since any mutation can cause the pointed node's"] # [doc = " source location to change, invalidating the pointer"] pub struct AstPtr < N : AstNode > { raw : SyntaxNodePtr < N :: Language > , }
    };
}

AstPtr!()