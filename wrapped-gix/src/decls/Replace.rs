macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! Replace {
    () => {
        deps!();
        # [derive (Debug , Clone)] struct Replace { find : BString , with : OwnShared < BString > , }
    };
}

Replace!();