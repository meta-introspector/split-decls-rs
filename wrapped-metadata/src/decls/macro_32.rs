macro_rules! deps {
    () => {
        MethodDef!();
    };
}

macro_rules! macro_32 {
    () => {
        deps!();
        code ! { MemberForwarded (1) (MethodDef , 1) }
    };
}

macro_32!()