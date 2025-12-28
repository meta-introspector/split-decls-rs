macro_rules! deps {
    () => {
        FloatLit!();
    };
}

macro_rules! macro_173 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , FloatLit , Float , FloatLit) ;
    };
}

macro_173!();