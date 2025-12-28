macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! validate_path_base_name {
    () => {
        deps!();
        pub (crate) fn validate_path_base_name (name : & str) -> Result < () > { validate_name (name , "path base name") }
    };
}

validate_path_base_name!()