macro_rules! deps {
    () => {
        CStringLit!();
    };
}

macro_rules! macro_178 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , CStringLit , CString , CStringLit) ;
    };
}

macro_178!();