macro_rules! IsPrefixOf {
    () => {
        pub (crate) trait IsPrefixOf < 'tcx > { fn is_prefix_of (& self , other : PlaceRef < 'tcx >) -> bool ; }
    };
}

IsPrefixOf!()