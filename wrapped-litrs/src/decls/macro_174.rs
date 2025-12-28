macro_rules! deps {
    () => {
        CharLit!();
    };
}

macro_rules! macro_174 {
    () => {
        deps!();
        helper_no_refs ! (impl_specific_lit_to_pm_lit , CharLit , Char , CharLit) ;
    };
}

macro_174!();