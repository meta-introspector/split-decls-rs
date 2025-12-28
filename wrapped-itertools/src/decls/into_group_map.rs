macro_rules! into_group_map {
    () => {
        # [doc = " Return a `HashMap` of keys mapped to a list of their corresponding values."] # [doc = ""] # [doc = " See [`.into_group_map()`](crate::Itertools::into_group_map)"] # [doc = " for more information."] pub fn into_group_map < I , K , V > (iter : I) -> HashMap < K , Vec < V > > where I : Iterator < Item = (K , V) > , K : Hash + Eq , { let mut lookup = HashMap :: < K , Vec < V > > :: new () ; iter . for_each (| (key , val) | { lookup . entry (key) . or_default () . push (val) ; }) ; lookup }
    };
}

into_group_map!()