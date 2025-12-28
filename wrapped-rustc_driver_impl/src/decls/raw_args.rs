macro_rules! raw_args {
    () => {
        # [doc = " Gets the raw unprocessed command-line arguments as Unicode strings, without doing any further"] # [doc = " processing (e.g., without `@file` expansion)."] # [doc = ""] # [doc = " This function is identical to [`env::args()`] except that it emits an error when it encounters"] # [doc = " non-Unicode arguments instead of panicking."] pub fn raw_args (early_dcx : & EarlyDiagCtxt) -> Vec < String > { let mut args = Vec :: new () ; let mut guar = Ok (()) ; for (i , arg) in env :: args_os () . enumerate () { match arg . into_string () { Ok (arg) => args . push (arg) , Err (arg) => { guar = Err (early_dcx . early_err (format ! ("argument {i} is not valid Unicode: {arg:?}"))) } } } if let Err (guar) = guar { guar . raise_fatal () ; } args }
    };
}

raw_args!()