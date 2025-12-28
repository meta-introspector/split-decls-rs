macro_rules! deps {
    () => {
        Transparent!();
    };
}

macro_rules! QueryKind {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] enum QueryKind { Input , Tracked , TrackedWithSalsaStruct , Transparent , Interned , }
    };
}

QueryKind!()