macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! opt_cstr {
    () => {
        deps!();
        fn opt_cstr < T : IntoCString > (o : Option < T >) -> Result < Option < CString > , Error > { match o { Some (s) => s . into_c_string () . map (Some) , None => Ok (None) , } }
    };
}

opt_cstr!()