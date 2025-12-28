macro_rules! concretize {
    () => {
        # [proc_macro_attribute] pub fn concretize (_attrs : proc_macro :: TokenStream , input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { input }
    };
}

concretize!()