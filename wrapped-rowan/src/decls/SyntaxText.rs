macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! SyntaxText {
    () => {
        deps!();
        # [derive (Clone)] pub struct SyntaxText { node : SyntaxNode , range : TextRange , }
    };
}

SyntaxText!()