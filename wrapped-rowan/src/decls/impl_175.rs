macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxNodePtr!();
        AstPtr!();
        Language!();
        AstNode!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < N : AstNode > AstPtr < N > { # [doc = " Returns an [`AstPtr`] for the node."] # [doc = ""] # [doc = " Panics if the provided node is mutable"] pub fn new (node : & N) -> Self { Self { raw : SyntaxNodePtr :: new (node . syntax ()) } } # [doc = " Like `Self::try_to_node` but panics on failure."] pub fn to_node (& self , root : & SyntaxNode < N :: Language >) -> N { self . try_to_node (root) . unwrap_or_else (| | panic ! ("can't resolve {self:?} with {root:?}")) } # [doc = " Given the root node containing the node `n` that `self` is a pointer to,"] # [doc = " returns `n` if possible. Panics if `root` is mutable. See [`SyntaxNodePtr::try_to_node`]."] pub fn try_to_node (& self , root : & SyntaxNode < N :: Language >) -> Option < N > { N :: cast (self . raw . try_to_node (root) ?) } # [doc = " Returns the underlying [`SyntaxNodePtr`]."] pub fn syntax_node_ptr (& self) -> SyntaxNodePtr < N :: Language > { self . raw } # [doc = " Casts this to an [`AstPtr`] to the given node type if possible."] pub fn cast < U : AstNode < Language = N :: Language > > (self) -> Option < AstPtr < U > > { if ! U :: can_cast (self . raw . kind) { return None ; } Some (AstPtr { raw : self . raw }) } }
    };
}

impl_175!()