macro_rules! deps {
    () => {
        PoloniusLocationTable!();
    };
}

macro_rules! FactWriter {
    () => {
        deps!();
        struct FactWriter < 'w > { location_table : & 'w PoloniusLocationTable , dir : & 'w Path , }
    };
}

FactWriter!();