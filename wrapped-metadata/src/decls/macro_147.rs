macro_rules! deps {
    () => {
        MethodDef!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        code ! { MemberForwarded (1) (MethodDef , 1) }
    };
}

macro_147!()