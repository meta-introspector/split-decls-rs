macro_rules! deps {
    () => {
        MemberRef!();
    };
}

macro_rules! macro_144 {
    () => {
        deps!();
        code ! { AttributeType (3) (MemberRef , 3) }
    };
}

macro_144!();