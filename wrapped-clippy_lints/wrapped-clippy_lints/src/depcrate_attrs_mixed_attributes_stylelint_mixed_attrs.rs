// Generated macro for lint_mixed_attrs (function)
macro_rules! Depcrate_attrs_mixed_attributes_stylelint_mixed_attrs {
() => {
// Module: crate::attrs::mixed_attributes_style
// Provides: {"lint_mixed_attrs"}
// Dependencies: {}
fn lint_mixed_attrs (cx : & EarlyContext < '_ > , attrs : & [Attribute]) { let mut attrs_iter = attrs . iter () . filter (| attr | ! attr . span . from_expansion ()) ; let span = if let (Some (first) , Some (last)) = (attrs_iter . next () , attrs_iter . next_back ()) { first . span . with_hi (last . span . hi ()) } else { return ; } ; span_lint (cx , MIXED_ATTRIBUTES_STYLE , span , "item has both inner and outer attributes" ,) ; }
};
}
