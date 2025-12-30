// Generated macro for impl_44 (impl)
macro_rules! Depcrate_cursorimpl_44 {
() => {
// Module: crate::cursor
// Provides: {"impl_44"}
// Dependencies: {}
impl SyntaxNodeChildren { fn new (parent : SyntaxNode) -> SyntaxNodeChildren { SyntaxNodeChildren { parent , next : None , next_initialized : false } } pub fn by_kind < F : Fn (SyntaxKind) -> bool > (self , matcher : F) -> SyntaxNodeChildrenByKind < F > { if ! self . next_initialized { SyntaxNodeChildrenByKind { next : self . parent . first_child_by_kind (& matcher) , matcher } } else { SyntaxNodeChildrenByKind { next : self . next . and_then (| node | { if matcher (node . kind ()) { Some (node) } else { node . next_sibling_by_kind (& matcher) } }) , matcher , } } } }
};
}
