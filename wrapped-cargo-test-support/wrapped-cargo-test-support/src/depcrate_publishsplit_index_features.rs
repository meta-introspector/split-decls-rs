// Generated macro for split_index_features (function)
macro_rules! Depcrate_publishsplit_index_features {
() => {
// Module: crate::publish
// Provides: {"split_index_features"}
// Dependencies: {}
fn split_index_features (mut features : FeatureMap) -> (FeatureMap , Option < FeatureMap >) { let mut features2 = FeatureMap :: new () ; for (feat , values) in features . iter_mut () { if values . iter () . any (| value | value . starts_with ("dep:") || value . contains ("?/")) { let new_values = std :: mem :: take (values) ; features2 . insert (feat . clone () , new_values) ; } } if features2 . is_empty () { (features , None) } else { (features , Some (features2)) } }
};
}
