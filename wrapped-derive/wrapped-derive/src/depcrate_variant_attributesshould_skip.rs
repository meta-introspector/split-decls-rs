// Generated macro for should_skip (function)
macro_rules! Depcrate_variant_attributesshould_skip {
() => {
// Module: crate::variant_attributes
// Provides: {"should_skip"}
// Dependencies: {}
fn should_skip (Variant { attrs , .. } : & Variant) -> bool { attrs . iter () . filter_map (| attr | { attr . path () . is_ident (ARBITRARY_ATTRIBUTE_NAME) . then (| | attr . parse_args :: < Meta > ()) . and_then (Result :: ok) }) . any (| meta | match meta { Meta :: Path (path) => path . is_ident ("skip") , _ => false , }) }
};
}
