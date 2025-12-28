macro_rules! deps {
    () => {
        TrackedChange!();
    };
}

macro_rules! ChangeListRef {
    () => {
        deps!();
        pub type ChangeListRef = [TrackedChange] ;
    };
}

ChangeListRef!();