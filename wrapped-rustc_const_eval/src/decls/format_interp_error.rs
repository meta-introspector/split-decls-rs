macro_rules! format_interp_error {
    () => {
        # [doc = " Turn the given error into a human-readable string. Expects the string to be printed, so if"] # [doc = " `RUSTC_CTFE_BACKTRACE` is set this will show a backtrace of the rustc internals that"] # [doc = " triggered the error."] # [doc = ""] # [doc = " This is NOT the preferred way to render an error; use `report` from `const_eval` instead."] # [doc = " However, this is useful when error messages appear in ICEs."] pub fn format_interp_error < 'tcx > (dcx : DiagCtxtHandle < '_ > , e : InterpErrorInfo < 'tcx >) -> String { let (e , backtrace) = e . into_parts () ; backtrace . print_backtrace () ; # [allow (rustc :: untranslatable_diagnostic)] let mut diag = dcx . struct_allow ("") ; let msg = e . diagnostic_message () ; e . add_args (& mut diag) ; let s = dcx . eagerly_translate_to_string (msg , diag . args . iter ()) ; diag . cancel () ; s }
    };
}

format_interp_error!()