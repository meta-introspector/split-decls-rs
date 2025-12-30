// Generated macro for impl_126 (impl)
macro_rules! Depcrate_standardimpl_126 {
() => {
// Module: crate::standard
// Provides: {"impl_126"}
// Dependencies: {}
impl Default for Config { fn default () -> Config { Config { colors : ColorSpecs :: default () , hyperlink : HyperlinkConfig :: default () , stats : false , heading : false , path : true , only_matching : false , per_match : false , per_match_one_line : false , replacement : Arc :: new (None) , max_columns : None , max_columns_preview : false , column : false , byte_offset : false , trim_ascii : false , separator_search : Arc :: new (None) , separator_context : Arc :: new (Some (b"--" . to_vec ())) , separator_field_match : Arc :: new (b":" . to_vec ()) , separator_field_context : Arc :: new (b"-" . to_vec ()) , separator_path : None , path_terminator : None , } } }
};
}
