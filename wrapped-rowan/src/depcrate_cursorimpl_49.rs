// Generated macro for impl_49 (impl)
macro_rules! Depcrate_cursorimpl_49 {
() => {
// Module: crate::cursor
// Provides: {"impl_49"}
// Dependencies: {}
impl SyntaxElementChildren { fn new (parent : SyntaxNode) -> SyntaxElementChildren { SyntaxElementChildren { parent , next : None , next_initialized : false } } pub fn by_kind < F : Fn (SyntaxKind) -> bool > (self , matcher : F) -> SyntaxElementChildrenByKind < F > { if ! self . next_initialized { SyntaxElementChildrenByKind { next : self . parent . first_child_or_token_by_kind (& matcher) , matcher , } } else { SyntaxElementChildrenByKind { next : self . next . and_then (| node | { if matcher (node . kind ()) { Some (node) } else { node . next_sibling_or_token_by_kind (& matcher) } }) , matcher , } } } }
};
}
