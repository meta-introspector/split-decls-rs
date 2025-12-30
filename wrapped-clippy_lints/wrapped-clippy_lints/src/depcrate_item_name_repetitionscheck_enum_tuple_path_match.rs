// Generated macro for check_enum_tuple_path_match (function)
macro_rules! Depcrate_item_name_repetitionscheck_enum_tuple_path_match {
() => {
// Module: crate::item_name_repetitions
// Provides: {"check_enum_tuple_path_match"}
// Dependencies: {}
# [doc = " Checks if an enum tuple variant contains a single field"] # [doc = " whose qualified path contains the variant's name."] fn check_enum_tuple_path_match (variant_name : & str , variant_data : VariantData < '_ >) -> bool { let VariantData :: Tuple (fields , ..) = variant_data else { return false ; } ; if fields . len () != 1 { return false ; } match fields [0] . ty . kind { TyKind :: Path (QPath :: Resolved (_ , path)) => path . segments . iter () . any (| segment | segment . ident . name . as_str () == variant_name) , TyKind :: Path (QPath :: TypeRelative (_ , segment)) => segment . ident . name . as_str () == variant_name , _ => false , } }
};
}
