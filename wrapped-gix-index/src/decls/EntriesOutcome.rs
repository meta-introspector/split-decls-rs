macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! EntriesOutcome {
    () => {
        deps!();
        struct EntriesOutcome { pub entries : Vec < Entry > , pub path_backing : Vec < u8 > , pub is_sparse : bool , }
    };
}

EntriesOutcome!();