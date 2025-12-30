// Generated macro for impl_tuple (macro)
macro_rules! Depcrate_schemaimpl_tuple {
() => {
// Module: crate::schema
// Provides: {"impl_tuple"}
// Dependencies: {}
macro_rules ! impl_tuple { ($ ($ name : ident) ,+) => { impl <$ ($ name) ,+> BorshSchema for ($ ($ name ,) +) where $ ($ name : BorshSchema) ,+ { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let elements = vec ! [$ ($ name :: declaration ()) ,+] ; let definition = Definition :: Tuple { elements } ; add_definition (Self :: declaration () , definition , definitions) ; $ ($ name :: add_definitions_recursively (definitions) ;) + } fn declaration () -> Declaration { let params = vec ! [$ ($ name :: declaration ()) ,+] ; if params . len () == 1 { format ! (r#"({},)"# , params [0]) } else { format ! (r#"({})"# , params . join (", ")) } } } } ; }
};
}
