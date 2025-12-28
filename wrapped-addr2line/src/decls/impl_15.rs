macro_rules! deps {
    () => {
        FunctionName!();
        Result!();
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < R : gimli :: Reader > FunctionName < R > { # [doc = " The raw name of this function before demangling."] pub fn raw_name (& self) -> Result < Cow < '_ , str > , Error > { self . name . to_string_lossy () } # [doc = " The name of this function after demangling (if applicable)."] pub fn demangle (& self) -> Result < Cow < '_ , str > , Error > { self . raw_name () . map (| x | demangle_auto (x , self . language)) } }
    };
}

impl_15!()