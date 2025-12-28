macro_rules! deps {
    () => {
        FeatureMap!();
    };
}

macro_rules! split_index_features {
    () => {
        deps!();
        fn split_index_features (mut features : FeatureMap) -> (FeatureMap , Option < FeatureMap >) { let mut features2 = FeatureMap :: new () ; for (feat , values) in features . iter_mut () { if values . iter () . any (| value | value . starts_with ("dep:") || value . contains ("?/")) { let new_values = std :: mem :: take (values) ; features2 . insert (feat . clone () , new_values) ; } } if features2 . is_empty () { (features , None) } else { (features , Some (features2)) } }
    };
}

split_index_features!()