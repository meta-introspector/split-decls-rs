macro_rules! deps {
    () => {
        NameValidationError!();
        Result!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < T : AsRef < str > > PathBaseName < T > { # [doc = " Validated path base name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_path_base_name (name . as_ref ()) ? ; Ok (Self (name)) } }
    };
}

impl_162!();