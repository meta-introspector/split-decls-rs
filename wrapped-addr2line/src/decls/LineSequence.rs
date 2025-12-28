macro_rules! deps {
    () => {
        LineRow!();
    };
}

macro_rules! LineSequence {
    () => {
        deps!();
        struct LineSequence { start : u64 , end : u64 , rows : Box < [LineRow] > , }
    };
}

LineSequence!()