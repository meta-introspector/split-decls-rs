macro_rules! deps {
    () => {
        NameValidationError!();
        Result!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < T : AsRef < str > > FeatureName < T > { # [doc = " Validated feature name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_feature_name (name . as_ref ()) ? ; Ok (Self (name)) } }
    };
}

impl_160!();