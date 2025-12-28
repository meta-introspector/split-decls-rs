macro_rules! deps {
    () => {
        Result!();
        NameValidationError!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T : AsRef < str > > RegistryName < T > { # [doc = " Validated registry name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_registry_name (name . as_ref ()) ? ; Ok (Self (name)) } }
    };
}

impl_156!()