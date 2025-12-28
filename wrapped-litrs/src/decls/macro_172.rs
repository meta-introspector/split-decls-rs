macro_rules! deps {
    () => {
        IntegerLit!();
    };
}

macro_rules! macro_172 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , IntegerLit , Integer , IntegerLit) ;
    };
}

macro_172!();