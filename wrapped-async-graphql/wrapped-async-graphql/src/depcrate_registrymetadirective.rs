// Generated macro for MetaDirective (struct)
macro_rules! Depcrate_registryMetaDirective {
() => {
// Module: crate::registry
// Provides: {"MetaDirective"}
// Dependencies: {}
pub struct MetaDirective { pub name : String , pub description : Option < String > , pub locations : Vec < __DirectiveLocation > , pub args : IndexMap < String , MetaInputValue > , pub is_repeatable : bool , pub visible : Option < MetaVisibleFn > , pub composable : Option < String > , }
};
}
