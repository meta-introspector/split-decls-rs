macro_rules! deps {
    () => {
        Header!();
        AixHeader!();
        AixFileHeader!();
        AixMemberOffset!();
    };
}

macro_rules! macro_1185 {
    () => {
        deps!();
        unsafe_impl_pod ! (Header , AixHeader , AixFileHeader , AixMemberOffset ,) ;
    };
}

macro_1185!()