macro_rules! init_stack_size {
    () => {
        fn init_stack_size (early_dcx : & EarlyDiagCtxt) -> usize { * STACK_SIZE . get_or_init (| | { env :: var_os ("RUST_MIN_STACK") . as_ref () . map (| os_str | os_str . to_string_lossy ()) . filter (| s | ! s . trim () . is_empty ()) . map (| s | { let s = s . trim () ; # [allow (rustc :: untranslatable_diagnostic , rustc :: diagnostic_outside_of_impl)] s . parse :: < usize > () . unwrap_or_else (| _ | { let mut err = early_dcx . early_struct_fatal (format ! (r#"`RUST_MIN_STACK` should be a number of bytes, but was "{s}""# ,)) ; err . note ("you can also unset `RUST_MIN_STACK` to use the default stack size") ; err . emit () }) }) . unwrap_or (DEFAULT_STACK_SIZE) }) }
    };
}

init_stack_size!()