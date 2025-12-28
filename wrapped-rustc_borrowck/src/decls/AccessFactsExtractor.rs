macro_rules! deps {
    () => {
        PoloniusFacts!();
        PoloniusLocationTable!();
    };
}

macro_rules! AccessFactsExtractor {
    () => {
        deps!();
        # [doc = " MIR visitor extracting point-wise facts about accesses."] struct AccessFactsExtractor < 'a , 'tcx > { facts : & 'a mut PoloniusFacts , move_data : & 'a MoveData < 'tcx > , location_table : & 'a PoloniusLocationTable , }
    };
}

AccessFactsExtractor!()