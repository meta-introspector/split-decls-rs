macro_rules! Expander {
    () => {
        # [derive (Debug , PartialEq , Eq)] struct Expander (proc_macro_api :: ProcMacro) ;
    };
}

Expander!();