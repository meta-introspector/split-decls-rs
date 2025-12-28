macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
    };
}

macro_rules! AstNode {
    () => {
        deps!();
        # [doc = " The main trait to go from untyped [`SyntaxNode`] to a typed AST. The"] # [doc = " conversion itself has zero runtime cost: AST and syntax nodes have exactly"] # [doc = " the same representation: a pointer to the tree root and a pointer to the"] # [doc = " node itself."] pub trait AstNode { type Language : Language ; fn can_cast (kind : < Self :: Language as Language > :: Kind) -> bool where Self : Sized ; fn cast (node : SyntaxNode < Self :: Language >) -> Option < Self > where Self : Sized ; fn syntax (& self) -> & SyntaxNode < Self :: Language > ; fn clone_for_update (& self) -> Self where Self : Sized , { Self :: cast (self . syntax () . clone_for_update ()) . unwrap () } fn clone_subtree (& self) -> Self where Self : Sized , { Self :: cast (self . syntax () . clone_subtree ()) . unwrap () } }
    };
}

AstNode!()