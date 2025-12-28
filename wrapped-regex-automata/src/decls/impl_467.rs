macro_rules! deps {
    () => {
        WhichCaptures!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl Default for WhichCaptures { fn default () -> WhichCaptures { WhichCaptures :: All } }
    };
}

impl_467!()