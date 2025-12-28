macro_rules! deps {
    () => {
        LineSequence!();
    };
}

macro_rules! Lines {
    () => {
        deps!();
        pub (crate) struct Lines { files : Box < [String] > , sequences : Box < [LineSequence] > , }
    };
}

Lines!();