// Generated macro for impl_170 (impl)
macro_rules! Depcrate_astimpl_170 {
() => {
// Module: crate::ast
// Provides: {"impl_170"}
// Dependencies: {}
impl fmt :: Display for InlineAsmTemplatePiece { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: String (s) => { for c in s . chars () { match c { '{' => f . write_str ("{{") ? , '}' => f . write_str ("}}") ? , _ => c . fmt (f) ? , } } Ok (()) } Self :: Placeholder { operand_idx , modifier : Some (modifier) , .. } => { write ! (f , "{{{operand_idx}:{modifier}}}") } Self :: Placeholder { operand_idx , modifier : None , .. } => { write ! (f , "{{{operand_idx}}}") } } } }
};
}
