macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! IID {
    () => {
        deps!();
        pub type IID = GUID ;
    };
}

IID!();