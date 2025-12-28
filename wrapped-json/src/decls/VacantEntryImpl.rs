macro_rules! deps {
    () => {
        Value!();
        VacantEntry!();
    };
}

macro_rules! VacantEntryImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type VacantEntryImpl < 'a > = indexmap :: map :: VacantEntry < 'a , String , Value > ;
    };
}

VacantEntryImpl!();