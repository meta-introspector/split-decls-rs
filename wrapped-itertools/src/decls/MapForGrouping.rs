macro_rules! deps {
    () => {
        MapSpecialCase!();
        GroupingMapFn!();
    };
}

macro_rules! MapForGrouping {
    () => {
        deps!();
        # [doc = " A wrapper to allow for an easy [`into_grouping_map_by`](crate::Itertools::into_grouping_map_by)"] pub type MapForGrouping < I , F > = MapSpecialCase < I , GroupingMapFn < F > > ;
    };
}

MapForGrouping!()