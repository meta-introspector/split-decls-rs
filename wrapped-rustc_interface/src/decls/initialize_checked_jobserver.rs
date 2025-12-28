macro_rules! initialize_checked_jobserver {
    () => {
        # [doc = " Initialize jobserver before getting `jobserver::client` and `build_session`."] pub (crate) fn initialize_checked_jobserver (early_dcx : & EarlyDiagCtxt) { jobserver :: initialize_checked (| err | { # [allow (rustc :: untranslatable_diagnostic)] # [allow (rustc :: diagnostic_outside_of_impl)] early_dcx . early_struct_warn (err) . with_note ("the build environment is likely misconfigured") . emit () }) ; }
    };
}

initialize_checked_jobserver!();