// Generated macro for check (function)
macro_rules! Depcrate_attrs_mixed_attributes_stylecheck {
() => {
// Module: crate::attrs::mixed_attributes_style
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , item_span : Span , attrs : & [Attribute]) { let mut inner_attr_kind : FxHashSet < SimpleAttrKind > = FxHashSet :: default () ; let mut outer_attr_kind : FxHashSet < SimpleAttrKind > = FxHashSet :: default () ; let source_map = cx . sess () . source_map () ; let item_src = source_map . lookup_source_file (item_span . lo ()) ; for attr in attrs { if attr . span . from_expansion () || ! attr_in_same_src_as_item (source_map , & item_src , attr . span) { continue ; } let kind : SimpleAttrKind = (& attr . kind) . into () ; match attr . style { AttrStyle :: Inner => { if outer_attr_kind . contains (& kind) { lint_mixed_attrs (cx , attrs) ; return ; } inner_attr_kind . insert (kind) ; } , AttrStyle :: Outer => { if inner_attr_kind . contains (& kind) { lint_mixed_attrs (cx , attrs) ; return ; } outer_attr_kind . insert (kind) ; } , } } }
};
}
