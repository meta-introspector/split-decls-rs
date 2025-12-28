macro_rules! deps {
    () => {
        OccupiedEntry!();
        Value!();
    };
}

macro_rules! OccupiedEntryImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type OccupiedEntryImpl < 'a > = indexmap :: map :: OccupiedEntry < 'a , String , Value > ;
    };
}

OccupiedEntryImpl!();