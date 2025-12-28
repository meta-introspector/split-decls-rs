macro_rules! deps {
    () => {
        FatArch64!();
        FatHeader!();
        FatArch32!();
    };
}

macro_rules! macro_4726 {
    () => {
        deps!();
        unsafe_impl_pod ! (FatHeader , FatArch32 , FatArch64 ,) ;
    };
}

macro_4726!()