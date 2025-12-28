macro_rules! deps {
    () => {
        AgileReferenceOptions!();
    };
}

macro_rules! AGILEREFERENCE_DEFAULT {
    () => {
        deps!();
        pub const AGILEREFERENCE_DEFAULT : AgileReferenceOptions = AgileReferenceOptions (0i32) ;
    };
}

AGILEREFERENCE_DEFAULT!();