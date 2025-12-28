macro_rules! proc_macro {
    () => {
        # [allow (rustc :: untranslatable_diagnostic)] pub mod proc_macro ;
    };
}

proc_macro!()