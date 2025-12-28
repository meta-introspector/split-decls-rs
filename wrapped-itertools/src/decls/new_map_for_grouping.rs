macro_rules! deps {
    () => {
        MapForGrouping!();
        MapSpecialCase!();
        GroupingMapFn!();
    };
}

macro_rules! new_map_for_grouping {
    () => {
        deps!();
        pub (crate) fn new_map_for_grouping < K , I : Iterator , F : FnMut (& I :: Item) -> K > (iter : I , key_mapper : F ,) -> MapForGrouping < I , F > { MapSpecialCase { iter , f : GroupingMapFn (key_mapper) , } }
    };
}

new_map_for_grouping!()