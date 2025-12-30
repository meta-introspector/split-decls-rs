// Generated macro for push_prefix_arguments (function)
macro_rules! Depcrate_fetch_refmap_initpush_prefix_arguments {
() => {
// Module: crate::fetch::refmap::init
// Provides: {"push_prefix_arguments"}
// Dependencies: {}
fn push_prefix_arguments (prefix_from_spec_as_filter_on_remote : bool , arguments : & mut Vec < BString > , all_refspecs : & [gix_refspec :: RefSpec] ,) { if ! prefix_from_spec_as_filter_on_remote { return ; } let mut seen = HashSet :: new () ; for spec in all_refspecs { let spec = spec . to_ref () ; if seen . insert (spec . instruction ()) { let mut prefixes = Vec :: with_capacity (1) ; spec . expand_prefixes (& mut prefixes) ; for mut prefix in prefixes { prefix . insert_str (0 , "ref-prefix ") ; arguments . push (prefix) ; } } } }
};
}
