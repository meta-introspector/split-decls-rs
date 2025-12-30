// Generated macro for NonLocalDefinitionsDiag (enum)
macro_rules! Depcrate_lintsNonLocalDefinitionsDiag {
() => {
// Module: crate::lints
// Provides: {"NonLocalDefinitionsDiag"}
// Dependencies: {}
pub (crate) enum NonLocalDefinitionsDiag { Impl { depth : u32 , body_kind_descr : & 'static str , body_name : String , cargo_update : Option < NonLocalDefinitionsCargoUpdateNote > , const_anon : Option < Option < Span > > , doctest : bool , macro_to_change : Option < (String , & 'static str) > , } , MacroRules { depth : u32 , body_kind_descr : & 'static str , body_name : String , doctest : bool , cargo_update : Option < NonLocalDefinitionsCargoUpdateNote > , } , }
};
}
