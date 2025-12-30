// Generated macro for field_visibilities_query (function)
macro_rules! Depcrate_visibilityfield_visibilities_query {
() => {
// Module: crate::visibility
// Provides: {"field_visibilities_query"}
// Dependencies: {}
# [doc = " Resolve visibility of all specific fields of a struct or union variant."] pub (crate) fn field_visibilities_query (db : & dyn DefDatabase , variant_id : VariantId ,) -> Arc < ArenaMap < LocalFieldId , Visibility > > { let variant_fields = variant_id . fields (db) ; let fields = variant_fields . fields () ; if fields . is_empty () { return Arc :: default () ; } let resolver = variant_id . module (db) . resolver (db) ; let mut res = ArenaMap :: default () ; for (field_id , field_data) in fields . iter () { res . insert (field_id , Visibility :: resolve (db , & resolver , & field_data . visibility)) ; } res . shrink_to_fit () ; Arc :: new (res) }
};
}
