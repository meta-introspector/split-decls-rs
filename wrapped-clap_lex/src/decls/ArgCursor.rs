macro_rules! deps {
    () => {
        RawArgs!();
    };
}

macro_rules! ArgCursor {
    () => {
        deps!();
        # [doc = " Position within [`RawArgs`]"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct ArgCursor { cursor : usize , }
    };
}

ArgCursor!();