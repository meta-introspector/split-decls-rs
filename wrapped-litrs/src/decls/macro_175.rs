macro_rules! deps {
    () => {
        StringLit!();
    };
}

macro_rules! macro_175 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , StringLit , String , StringLit) ;
    };
}

macro_175!()