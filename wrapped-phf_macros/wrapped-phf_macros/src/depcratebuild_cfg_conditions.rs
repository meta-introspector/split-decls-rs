// Generated macro for build_cfg_conditions (function)
macro_rules! Depcratebuild_cfg_conditions {
() => {
// Module: crate
// Provides: {"build_cfg_conditions"}
// Dependencies: {}
# [doc = " Generate conditional cfg conditions for a given mask and conditional entries"] fn build_cfg_conditions (mask : usize , conditional : & [& Entry]) -> Vec < proc_macro2 :: TokenStream > { let mut conditions = Vec :: new () ; for (i , & entry) in conditional . iter () . enumerate () { let include = (mask & (1 << i)) != 0 ; if let Some (attr) = entry . attrs . first () { if let Ok (meta) = attr . meta . require_list () { let tokens = & meta . tokens ; if include { conditions . push (quote ! (cfg ! (# tokens))) ; } else { conditions . push (quote ! (! cfg ! (# tokens))) ; } } } } conditions }
};
}
