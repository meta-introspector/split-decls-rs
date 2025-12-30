// Generated macro for impl_313 (impl)
macro_rules! Depcrate_target_featureimpl_313 {
() => {
// Module: crate::target_feature
// Provides: {"impl_313"}
// Dependencies: {}
impl TargetFeatures { pub fn from_attrs (attrs : & Attrs) -> Self { let mut result = TargetFeatures :: from_attrs_no_implications (attrs) ; result . expand_implications () ; result } fn expand_implications (& mut self) { let all_implications = LazyLock :: force (& TARGET_FEATURE_IMPLICATIONS) ; let mut queue = self . enabled . iter () . cloned () . collect :: < Vec < _ > > () ; while let Some (feature) = queue . pop () { if let Some (implications) = all_implications . get (& feature) { for implication in implications { if self . enabled . insert (implication . clone ()) { queue . push (implication . clone ()) ; } } } } } # [doc = " Retrieves the target features from the attributes, and does not expand the target features implied by them."] pub (crate) fn from_attrs_no_implications (attrs : & Attrs) -> Self { let enabled = attrs . by_key (sym :: target_feature) . tt_values () . filter_map (| tt | match tt . token_trees () . flat_tokens () { [tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (enable_ident)) , tt :: TokenTree :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : '=' , .. })) , tt :: TokenTree :: Leaf (tt :: Leaf :: Literal (tt :: Literal { kind : tt :: LitKind :: Str , symbol : features , .. })) ,] if enable_ident . sym == sym :: enable => Some (features) , _ => None , }) . flat_map (| features | features . as_str () . split (',') . map (Symbol :: intern)) . collect () ; Self { enabled } } }
};
}
