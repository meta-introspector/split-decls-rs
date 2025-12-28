macro_rules! deps {
    () => {
        ContainerAttributes!();
    };
}

macro_rules! DeriveEnum {
    () => {
        deps!();
        pub (crate) struct DeriveEnum { pub variants : Vec < EnumVariant > , pub attributes : ContainerAttributes , }
    };
}

DeriveEnum!();