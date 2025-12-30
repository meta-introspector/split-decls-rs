// Generated macro for Memmem (struct)
macro_rules! Depcrate_util_prefilter_memmemMemmem {
() => {
// Module: crate::util::prefilter::memmem
// Provides: {"Memmem"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct Memmem { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] _unused : () , # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] finder : memchr :: memmem :: Finder < 'static > , }
};
}
