// Generated macro for complete_name (function)
macro_rules! Depcrate_completionscomplete_name {
() => {
// Module: crate::completions
// Provides: {"complete_name"}
// Dependencies: {}
pub (super) fn complete_name (acc : & mut Completions , ctx : & CompletionContext < '_ > , NameContext { name , kind } : & NameContext ,) { match kind { NameKind :: Const => { item_list :: trait_impl :: complete_trait_impl_const (acc , ctx , name) ; } NameKind :: Function => { item_list :: trait_impl :: complete_trait_impl_fn (acc , ctx , name) ; } NameKind :: IdentPat (pattern_ctx) => { if ctx . token . kind () != syntax :: T ! [_] { complete_patterns (acc , ctx , pattern_ctx) } } NameKind :: Module (mod_under_caret) => { mod_ :: complete_mod (acc , ctx , mod_under_caret) ; } NameKind :: TypeAlias => { item_list :: trait_impl :: complete_trait_impl_type_alias (acc , ctx , name) ; } NameKind :: RecordField => { field :: complete_field_list_record_variant (acc , ctx) ; } NameKind :: TypeParam => { acc . add_keyword_snippet (ctx , "const" , "const $1: $0") ; } NameKind :: ConstParam | NameKind :: Enum | NameKind :: MacroDef | NameKind :: MacroRules | NameKind :: Rename | NameKind :: SelfParam | NameKind :: Static | NameKind :: Struct | NameKind :: Trait | NameKind :: Union | NameKind :: Variant => () , } }
};
}
