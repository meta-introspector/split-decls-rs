// Generated macro for field_alias_mapper (macro)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperfield_alias_mapper {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"field_alias_mapper"}
// Dependencies: {}
macro_rules ! field_alias_mapper { ($ ($ Tuple : tt { $ (($ idx : tt) -> $ T : ident , $ ST : ident , $ TT : ident ,) + }) +) => { $ (impl < _S , $ ($ T ,) *> FieldAliasMapper < _S > for ($ ($ T ,) *) where _S : AliasSource , $ ($ T : FieldAliasMapper < _S >,) * { type Out = ($ (<$ T as FieldAliasMapper < _S >>:: Out ,) *) ; fn map (self , alias : & Alias < _S >) -> Self :: Out { ($ (self .$ idx . map (alias) ,) *) } }) * } }
};
}
