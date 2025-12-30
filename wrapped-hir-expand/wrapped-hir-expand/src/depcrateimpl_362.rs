// Generated macro for impl_362 (impl)
macro_rules! Depcrateimpl_362 {
() => {
// Module: crate
// Provides: {"impl_362"}
// Dependencies: {}
impl MacroDefKind { # [inline] pub fn is_declarative (& self) -> bool { matches ! (self , MacroDefKind :: Declarative (..)) } pub fn erased_ast_id (& self) -> ErasedAstId { match * self { MacroDefKind :: ProcMacro (id , ..) => id . erase () , MacroDefKind :: BuiltIn (id , _) | MacroDefKind :: BuiltInAttr (id , _) | MacroDefKind :: BuiltInDerive (id , _) | MacroDefKind :: BuiltInEager (id , _) | MacroDefKind :: Declarative (id , ..) => id . erase () , } } }
};
}
