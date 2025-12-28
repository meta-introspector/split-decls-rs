macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! validate_package_name {
    () => {
        deps!();
        pub (crate) fn validate_package_name (name : & str) -> Result < () > { for part in name . split ("::") { validate_name (part , "package name") ? ; } Ok (()) }
    };
}

validate_package_name!()