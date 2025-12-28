macro_rules! deps {
    () => {
        ByteLit!();
    };
}

macro_rules! macro_176 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , ByteLit , Byte , ByteLit) ;
    };
}

macro_176!()