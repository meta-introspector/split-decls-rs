// Generated macro for find_serde_attr_item (function)
macro_rules! Depcrate_derive_deserialize_allowing_unknownfind_serde_attr_item {
() => {
// Module: crate::derive_deserialize_allowing_unknown
// Provides: {"find_serde_attr_item"}
// Dependencies: {}
fn find_serde_attr_item (tcx : TyCtxt < '_ > , hir_id : HirId) -> Option < & TokenStream > { tcx . hir_attrs (hir_id) . iter () . find_map (| attribute | { if let Attribute :: Unparsed (attr_item) = attribute && let AttrItem { path : AttrPath { segments , .. } , args : AttrArgs :: Delimited (DelimArgs { tokens , .. }) , style : AttrStyle :: Outer , .. } = & * * attr_item && segments . len () == 1 && segments [0] . as_str () == "serde" { Some (tokens) } else { None } }) }
};
}
