// Generated macro for impl_189 (impl)
macro_rules! Depcrate_serde_implsimpl_189 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_189"}
// Dependencies: {}
impl < L : Language > Serialize for Children < & '_ SyntaxNode < L > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_seq (None) ? ; self . 0 . children_with_tokens () . try_for_each (| element | match element { NodeOrToken :: Node (it) => state . serialize_element (& it) , NodeOrToken :: Token (it) => state . serialize_element (& it) , }) ? ; state . end () } }
};
}
