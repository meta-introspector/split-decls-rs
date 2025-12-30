// Generated macro for attr_in_same_src_as_item (function)
macro_rules! Depcrate_attrs_mixed_attributes_styleattr_in_same_src_as_item {
() => {
// Module: crate::attrs::mixed_attributes_style
// Provides: {"attr_in_same_src_as_item"}
// Dependencies: {}
fn attr_in_same_src_as_item (source_map : & SourceMap , item_src : & Arc < SourceFile > , attr_span : Span) -> bool { let attr_src = source_map . lookup_source_file (attr_span . lo ()) ; Arc :: ptr_eq (item_src , & attr_src) }
};
}
