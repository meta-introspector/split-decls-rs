// Generated macro for has_non_exhaustive_attr (function)
macro_rules! Depcrate_attrshas_non_exhaustive_attr {
() => {
// Module: crate::attrs
// Provides: {"has_non_exhaustive_attr"}
// Dependencies: {}
# [doc = " Checks whether the given ADT, or any of its fields/variants, are marked as `#[non_exhaustive]`"] pub fn has_non_exhaustive_attr (tcx : TyCtxt < '_ > , adt : AdtDef < '_ >) -> bool { adt . is_variant_list_non_exhaustive () || find_attr ! (tcx . get_all_attrs (adt . did ()) , AttributeKind :: NonExhaustive (..)) || adt . variants () . iter () . any (| variant_def | { variant_def . is_field_list_non_exhaustive () || find_attr ! (tcx . get_all_attrs (variant_def . def_id) , AttributeKind :: NonExhaustive (..)) }) || adt . all_fields () . any (| field_def | find_attr ! (tcx . get_all_attrs (field_def . did) , AttributeKind :: NonExhaustive (..))) }
};
}
