macro_rules! deps {
    () => {
        AixFileHeader!();
        AixMemberOffset!();
        Header!();
        AixHeader!();
    };
}

macro_rules! macro_1185 {
    () => {
        deps!();
        unsafe_impl_pod ! (Header , AixHeader , AixFileHeader , AixMemberOffset ,) ;
    };
}

macro_1185!();