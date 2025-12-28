macro_rules! mk_map {
    () => {
        fn mk_map < K : Ord , V > (entries : Vec < (K , V) >) -> BTreeMap < K , V > { BTreeMap :: from_iter (entries . into_iter ()) }
    };
}

mk_map!();