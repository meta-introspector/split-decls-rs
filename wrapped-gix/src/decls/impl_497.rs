macro_rules! deps {
    () => {
        Options!();
        Default!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { object_store_slots : Default :: default () , permissions : Default :: default () , git_dir_trust : None , filter_config_section : None , lossy_config : false , lenient_config : true , bail_if_untrusted : false , open_path_as_is : false , api_config_overrides : Vec :: new () , cli_config_overrides : Vec :: new () , current_dir : None , } } }
    };
}

impl_497!()