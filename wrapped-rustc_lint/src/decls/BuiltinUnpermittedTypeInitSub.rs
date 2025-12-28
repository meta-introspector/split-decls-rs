macro_rules! deps {
    () => {
        InitError!();
    };
}

macro_rules! BuiltinUnpermittedTypeInitSub {
    () => {
        deps!();
        pub (crate) struct BuiltinUnpermittedTypeInitSub { pub err : InitError , }
    };
}

BuiltinUnpermittedTypeInitSub!()