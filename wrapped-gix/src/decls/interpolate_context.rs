macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! interpolate_context {
    () => {
        deps!();
        pub (crate) fn interpolate_context < 'a > (git_install_dir : Option < & 'a std :: path :: Path > , home_dir : Option < & 'a std :: path :: Path > ,) -> gix_config :: path :: interpolate :: Context < 'a > { gix_config :: path :: interpolate :: Context { git_install_dir , home_dir , home_for_user : Some (gix_config :: path :: interpolate :: home_for_user) , } }
    };
}

interpolate_context!();