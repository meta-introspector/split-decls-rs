// Generated macro for impl_36 (impl)
macro_rules! Depcrate_cursorimpl_36 {
() => {
// Module: crate::cursor
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Display for SyntaxNode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . preorder_with_tokens () . filter_map (| event | match event { WalkEvent :: Enter (NodeOrToken :: Token (token)) => Some (token) , _ => None , }) . try_for_each (| it | fmt :: Display :: fmt (& it , f)) } }
};
}
