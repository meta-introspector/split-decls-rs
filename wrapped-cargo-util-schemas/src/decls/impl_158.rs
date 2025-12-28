macro_rules! deps {
    () => {
        NameValidationError!();
        Result!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T : AsRef < str > > ProfileName < T > { # [doc = " Validated profile name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_profile_name (name . as_ref ()) ? ; Ok (Self (name)) } }
    };
}

impl_158!()