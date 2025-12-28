macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! into_opt_c_string {
    () => {
        deps!();
        pub fn into_opt_c_string < S > (opt_s : Option < S >) -> Result < Option < CString > , Error > where S : IntoCString , { match opt_s { None => Ok (None) , Some (s) => Ok (Some (s . into_c_string () ?)) , } }
    };
}

into_opt_c_string!()