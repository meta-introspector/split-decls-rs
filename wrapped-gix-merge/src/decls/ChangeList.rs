macro_rules! deps {
    () => {
        TrackedChange!();
    };
}

macro_rules! ChangeList {
    () => {
        deps!();
        pub type ChangeList = Vec < TrackedChange > ;
    };
}

ChangeList!()