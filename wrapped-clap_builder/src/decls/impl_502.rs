macro_rules! deps {
    () => {
        MatchedArg!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl Eq for MatchedArg { }
    };
}

impl_502!()