macro_rules! deps {
    () => {
        Expander!();
    };
}

macro_rules! arg_expand_all {
    () => {
        deps!();
        # [doc = " Replaces any `@file` arguments with the contents of `file`, with each line of `file` as a"] # [doc = " separate argument."] # [doc = ""] # [doc = " **Note:** This function doesn't interpret argument 0 in any special way."] # [doc = " If this function is intended to be used with command line arguments,"] # [doc = " `argv[0]` must be removed prior to calling it manually."] # [allow (rustc :: untranslatable_diagnostic)] pub fn arg_expand_all (early_dcx : & EarlyDiagCtxt , at_args : & [String]) -> Vec < String > { let mut expander = Expander :: default () ; let mut result = Ok (()) ; for arg in at_args { if let Err (err) = expander . arg (arg) { result = Err (early_dcx . early_err (format ! ("failed to load argument file: {err}"))) ; } } if let Err (guar) = result { guar . raise_fatal () ; } expander . finish () }
    };
}

arg_expand_all!();