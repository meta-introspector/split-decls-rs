// Generated macro for impl_27 (impl)
macro_rules! Depcrate_file_commitimpl_27 {
() => {
// Module: crate::file::commit
// Provides: {"impl_27"}
// Dependencies: {}
impl Debug for Commit < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Commit {{ id: {}, lex_pos: {}, generation: {}, root_tree_id: {}, parent1: {:?}, parent2: {:?} }}" , self . id () , self . pos , self . generation () , self . root_tree_id () , self . parent1 , self . parent2 ,) } }
};
}
