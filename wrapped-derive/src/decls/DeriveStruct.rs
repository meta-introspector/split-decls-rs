macro_rules! deps {
    () => {
        ContainerAttributes!();
    };
}

macro_rules! DeriveStruct {
    () => {
        deps!();
        pub (crate) struct DeriveStruct { pub fields : Option < Fields > , pub attributes : ContainerAttributes , }
    };
}

DeriveStruct!();