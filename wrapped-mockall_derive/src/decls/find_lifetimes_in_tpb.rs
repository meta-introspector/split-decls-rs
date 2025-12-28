macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! find_lifetimes_in_tpb {
    () => {
        deps!();
        fn find_lifetimes_in_tpb (bound : & TypeParamBound) -> HashSet < Lifetime > { let mut ret = HashSet :: default () ; match bound { TypeParamBound :: Lifetime (lt) => { ret . insert (lt . clone ()) ; } , TypeParamBound :: Trait (tb) => { ret . extend (find_lifetimes_in_path (& tb . path)) ; } , _ => () } ; ret }
    };
}

find_lifetimes_in_tpb!();