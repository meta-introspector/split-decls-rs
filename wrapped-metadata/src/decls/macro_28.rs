macro_rules! deps {
    () => {
        MemberRef!();
        MethodDef!();
    };
}

macro_rules! macro_28 {
    () => {
        deps!();
        code ! { AttributeType (3) (MethodDef , 2) (MemberRef , 3) }
    };
}

macro_28!();