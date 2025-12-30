// Generated macro for impl_271 (impl)
macro_rules! Depcrate_lang_itemimpl_271 {
() => {
// Module: crate::lang_item
// Provides: {"impl_271"}
// Dependencies: {}
impl LangItemTarget { pub fn as_enum (self) -> Option < EnumId > { match self { LangItemTarget :: EnumId (id) => Some (id) , _ => None , } } pub fn as_function (self) -> Option < FunctionId > { match self { LangItemTarget :: Function (id) => Some (id) , _ => None , } } pub fn as_impl_def (self) -> Option < ImplId > { match self { LangItemTarget :: ImplDef (id) => Some (id) , _ => None , } } pub fn as_static (self) -> Option < StaticId > { match self { LangItemTarget :: Static (id) => Some (id) , _ => None , } } pub fn as_struct (self) -> Option < StructId > { match self { LangItemTarget :: Struct (id) => Some (id) , _ => None , } } pub fn as_trait (self) -> Option < TraitId > { match self { LangItemTarget :: Trait (id) => Some (id) , _ => None , } } pub fn as_enum_variant (self) -> Option < EnumVariantId > { match self { LangItemTarget :: EnumVariant (id) => Some (id) , _ => None , } } pub fn as_type_alias (self) -> Option < TypeAliasId > { match self { LangItemTarget :: TypeAlias (id) => Some (id) , _ => None , } } pub fn as_adt (self) -> Option < AdtId > { match self { LangItemTarget :: Union (it) => Some (it . into ()) , LangItemTarget :: EnumId (it) => Some (it . into ()) , LangItemTarget :: Struct (it) => Some (it . into ()) , _ => None , } } }
};
}
