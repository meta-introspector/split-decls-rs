// Generated macro for impl_725 (impl)
macro_rules! Depcrate_open_optionsimpl_725 {
() => {
// Module: crate::open::options
// Provides: {"impl_725"}
// Dependencies: {}
impl gix_sec :: trust :: DefaultForLevel for Options { fn default_for_level (level : gix_sec :: Trust) -> Self { match level { gix_sec :: Trust :: Full => Options { object_store_slots : Default :: default () , permissions : Permissions :: default_for_level (level) , git_dir_trust : gix_sec :: Trust :: Full . into () , filter_config_section : Some (config :: section :: is_trusted) , lossy_config : false , bail_if_untrusted : false , lenient_config : true , open_path_as_is : false , api_config_overrides : Vec :: new () , cli_config_overrides : Vec :: new () , current_dir : None , } , gix_sec :: Trust :: Reduced => Options { object_store_slots : gix_odb :: store :: init :: Slots :: Given (32) , permissions : Permissions :: default_for_level (level) , git_dir_trust : gix_sec :: Trust :: Reduced . into () , filter_config_section : Some (config :: section :: is_trusted) , bail_if_untrusted : false , lenient_config : true , open_path_as_is : false , lossy_config : false , api_config_overrides : Vec :: new () , cli_config_overrides : Vec :: new () , current_dir : None , } , } } }
};
}
