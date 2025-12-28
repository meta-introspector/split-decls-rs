macro_rules! deps {
    () => {
        Error!();
        FetchRecurse!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl FetchRecurse { # [doc = " Check if `boolean` is set and translate it the respective variant, or check the underlying string"] # [doc = " value for non-boolean options."] # [doc = " On error, it returns the obtained string value which would be the invalid value."] pub fn new (boolean : Result < bool , gix_config :: value :: Error >) -> Result < Self , BString > { Ok (match boolean { Ok (value) => { if value { FetchRecurse :: Always } else { FetchRecurse :: Never } } Err (err) => { if err . input != "on-demand" { return Err (err . input) ; } FetchRecurse :: OnDemand } }) } }
    };
}

impl_7!();