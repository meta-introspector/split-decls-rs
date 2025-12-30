// Generated macro for impl_950 (impl)
macro_rules! Depcrateimpl_950 {
() => {
// Module: crate
// Provides: {"impl_950"}
// Dependencies: {}
impl Complete { pub fn extract (is_trait : bool , attrs : & Attrs) -> Complete { let mut do_not_complete = Complete :: Yes ; for ra_attr in attrs . rust_analyzer_tool () { let segments = ra_attr . path . segments () ; if segments . len () != 2 { continue ; } let action = segments [1] . symbol () ; if * action == sym :: completions { match ra_attr . token_tree_value () . map (| tt | tt . token_trees () . flat_tokens ()) { Some ([tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (ident))]) => { if ident . sym == sym :: ignore_flyimport { do_not_complete = Complete :: IgnoreFlyimport ; } else if is_trait { if ident . sym == sym :: ignore_methods { do_not_complete = Complete :: IgnoreMethods ; } else if ident . sym == sym :: ignore_flyimport_methods { do_not_complete = Complete :: IgnoreFlyimportMethods ; } } } _ => { } } } } do_not_complete } # [inline] pub fn for_trait_item (trait_attr : Complete , item_attr : Complete) -> Complete { match (trait_attr , item_attr) { (Complete :: IgnoreFlyimportMethods | Complete :: IgnoreFlyimport | Complete :: IgnoreMethods , _ ,) => Complete :: IgnoreFlyimport , _ => item_attr , } } }
};
}
