macro_rules! QueryKind {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] enum QueryKind { Input , Tracked , TrackedWithSalsaStruct , Transparent , Interned , }
    };
}

QueryKind!()