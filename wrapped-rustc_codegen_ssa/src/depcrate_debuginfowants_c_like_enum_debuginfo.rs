// Generated macro for wants_c_like_enum_debuginfo (function)
macro_rules! Depcrate_debuginfowants_c_like_enum_debuginfo {
() => {
// Module: crate::debuginfo
// Provides: {"wants_c_like_enum_debuginfo"}
// Dependencies: {}
# [doc = " Returns true if we want to generate a DW_TAG_enumeration_type description for"] # [doc = " this instead of a DW_TAG_struct_type with DW_TAG_variant_part."] # [doc = ""] # [doc = " NOTE: This is somewhat inconsistent right now: For empty enums and enums with a single"] # [doc = "       fieldless variant, we generate DW_TAG_struct_type, although a"] # [doc = "       DW_TAG_enumeration_type would be a better fit."] pub fn wants_c_like_enum_debuginfo < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > ,) -> bool { match enum_type_and_layout . ty . kind () { ty :: Adt (adt_def , _) => { if ! adt_def . is_enum () { return false ; } if type_names :: cpp_like_debuginfo (tcx) && tag_base_type_opt (tcx , enum_type_and_layout) . map (| ty | ty . primitive_size (tcx) . bits ()) == Some (128) { return false ; } match adt_def . variants () . len () { 0 => false , 1 => { enum_type_and_layout . size != Size :: ZERO && adt_def . all_fields () . count () == 0 } _ => { adt_def . all_fields () . count () == 0 } } } _ => false , } }
};
}
