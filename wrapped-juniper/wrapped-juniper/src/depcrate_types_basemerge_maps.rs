// Generated macro for merge_maps (function)
macro_rules! Depcrate_types_basemerge_maps {
() => {
// Module: crate::types::base
// Provides: {"merge_maps"}
// Dependencies: {}
# [doc = " Merges `src` object's fields into `dest`"] fn merge_maps < S > (dest : & mut Object < S > , src : Object < S >) { for (key , value) in src { if dest . contains_field (& key) { merge_key_into (dest , & key , value) ; } else { dest . add_field (key , value) ; } } }
};
}
