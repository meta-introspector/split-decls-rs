macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! slice {
    () => {
        deps!();
        pub fn slice (range : Range) -> & [char] { & range . doc [range . offset .. range . offset + range . len] }
    };
}

slice!();