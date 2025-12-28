macro_rules! deps {
    () => {
        Error!();
        Entry!();
        EntryStatus!();
    };
}

macro_rules! StatusResult {
    () => {
        deps!();
        type StatusResult < 'index , T , U > = Result < (& 'index gix_index :: Entry , usize , & 'index BStr , EntryStatus < T , U >) , Error > ;
    };
}

StatusResult!();