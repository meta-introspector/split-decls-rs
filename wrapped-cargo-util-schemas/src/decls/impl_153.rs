macro_rules! deps {
    () => {
        NameValidationError!();
        Result!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T : AsRef < str > > PackageName < T > { # [doc = " Validated package name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_package_name (name . as_ref ()) ? ; Ok (Self (name)) } }
    };
}

impl_153!()