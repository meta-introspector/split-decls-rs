macro_rules! deps {
    () => {
        DWORD!();
    };
}

macro_rules! LCID {
    () => {
        deps!();
        pub type LCID = DWORD ;
    };
}

LCID!()