// Generated macro for build_by_item_enum_core (function)
macro_rules! Depcrate_item_typebuild_by_item_enum_core {
() => {
// Module: crate::item_type
// Provides: {"build_by_item_enum_core"}
// Dependencies: {}
fn build_by_item_enum_core (attr : Option < TokenStream > , item : & ItemEnum , kinds : & mut HelperAttributeKinds ,) -> Result < TokenStream > { let es = DeriveEntry :: from_root (attr , & item . attrs) ? ; kinds . extend (& es) ; let hattrs = HelperAttributes :: from_attrs (& item . attrs , AttributeTarget :: Type , & kinds . without_derive_ex () ,) ? ; let variants = VariantEntry :: from_variants (& item . variants , kinds) ? ; let mut ts_all = TokenStream :: new () ; for e in es { let result = match e . kind { DeriveItemKind :: CompareOp (op) => { build_compare_op_for_enum (op , item , & e , & hattrs , & variants) } DeriveItemKind :: Copy => build_copy_for_enum (item , & e , & variants) , DeriveItemKind :: Clone => build_clone_for_enum (item , & e , & variants) , DeriveItemKind :: Debug => build_debug_for_enum (item , & e , & hattrs , & variants) , DeriveItemKind :: Default => build_default_for_enum (item , & e , & hattrs , & variants) , _ => bail ! (e . span , "derive `{}` for enum is not supported" , e . kind) , } ; ts_all . extend (e . apply_dump (result)) ; } Ok (ts_all) }
};
}
