// Generated macro for SyntaxExtensionTrait (trait)
macro_rules! Depcrate_syntax_extension_traitSyntaxExtensionTrait {
() => {
// Module: crate::syntax_extension_trait
// Provides: {"SyntaxExtensionTrait"}
// Dependencies: {}
pub trait SyntaxExtensionTrait < DRT : OpaqueDeriveResolution + 'static > : Send + Sync + 'static + Sized { fn new_macro_extension (sess : & Session , kind : Arc < dyn MacroKindTrait < DRT > > , span : Span , allow_internal_attrs : Vec < Attribute > , edition : Edition , name : Symbol , attrs : & [Attribute] , is_local : bool ,) -> Self ; fn dummy_bang_extension (edition : Edition) -> Self ; fn dummy_derive_extension (edition : Edition) -> Self ; fn non_macro_attr_extension (edition : Edition) -> Self ; fn glob_delegation_extension (def_id : DefId , impl_def_id : Option < DefId > , edition : Edition) -> Self ; fn get_macro_kind (& self) -> & dyn MacroKindTrait < DRT > ; fn as_bang (& self) -> Option < & dyn Any > ; fn as_attr (& self) -> Option < & dyn Any > ; fn as_legacy_bang (& self) -> Option < & dyn Any > ; fn as_derive (& self) -> Option < & dyn Any > ; fn as_legacy_derive (& self) -> Option < & dyn Any > ; fn as_glob_delegation (& self) -> Option < & dyn Any > ; }
};
}
