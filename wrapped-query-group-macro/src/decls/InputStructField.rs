macro_rules! InputStructField {
    () => {
        # [derive (Debug)] struct InputStructField { name : proc_macro2 :: TokenStream , ty : proc_macro2 :: TokenStream , }
    };
}

InputStructField!();