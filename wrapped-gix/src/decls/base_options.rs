macro_rules! deps {
    () => {
        Default!();
        Options!();
    };
}

macro_rules! base_options {
    () => {
        deps!();
        pub (crate) fn base_options (lossy : bool , lenient : bool) -> gix_config :: file :: init :: Options < 'static > { gix_config :: file :: init :: Options { lossy , ignore_io_errors : lenient , .. Default :: default () } }
    };
}

base_options!();