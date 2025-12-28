macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! validate_registry_name {
    () => {
        deps!();
        pub (crate) fn validate_registry_name (name : & str) -> Result < () > { validate_name (name , "registry name") }
    };
}

validate_registry_name!();