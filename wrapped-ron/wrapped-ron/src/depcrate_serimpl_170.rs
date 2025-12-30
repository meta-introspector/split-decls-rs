// Generated macro for impl_170 (impl)
macro_rules! Depcrate_serimpl_170 {
() => {
// Module: crate::ser
// Provides: {"impl_170"}
// Dependencies: {}
impl Default for PrettyConfig { fn default () -> Self { PrettyConfig { depth_limit : usize :: MAX , new_line : if cfg ! (not (target_os = "windows")) { Cow :: Borrowed ("\n") } else { Cow :: Borrowed ("\r\n") } , indentor : Cow :: Borrowed ("    ") , separator : Cow :: Borrowed (" ") , struct_names : false , separate_tuple_members : false , enumerate_arrays : false , extensions : Extensions :: empty () , compact_arrays : false , escape_strings : true , compact_structs : false , compact_maps : false , number_suffixes : false , path_meta : None , } } }
};
}
