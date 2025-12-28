macro_rules! deps {
    () => {
        VacantEntry!();
        Value!();
    };
}

macro_rules! VacantEntryImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type VacantEntryImpl < 'a > = indexmap :: map :: VacantEntry < 'a , String , Value > ;
    };
}

VacantEntryImpl!()