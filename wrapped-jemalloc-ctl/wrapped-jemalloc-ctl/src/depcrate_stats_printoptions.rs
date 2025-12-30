// Generated macro for Options (struct)
macro_rules! Depcrate_stats_printOptions {
() => {
// Module: crate::stats_print
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Statistics configuration."] # [doc = ""] # [doc = " All options default to `false`."] # [derive (Copy , Clone , Default)] pub struct Options { # [doc = " If set, the output will be JSON-formatted."] # [doc = ""] # [doc = " This corresponds to the `J` character."] pub json_format : bool , # [doc = " If set, information that never changes during execution will be skipped."] # [doc = ""] # [doc = " This corresponds to the `g` character."] pub skip_constants : bool , # [doc = " If set, merged information about arenas will be skipped."] # [doc = ""] # [doc = " This corresponds to the `m` character."] pub skip_merged_arenas : bool , # [doc = " If set, information about individual arenas will be skipped."] # [doc = ""] # [doc = " This corresponds to the `a` character."] pub skip_per_arena : bool , # [doc = " If set, information about individual size classes for bins will be skipped."] # [doc = ""] # [doc = " This corresponds to the `b` character."] pub skip_bin_size_classes : bool , # [doc = " If set, information about individual size classes for large objects will be skipped."] # [doc = ""] # [doc = " This corresponds to the `l` character."] pub skip_large_size_classes : bool , # [doc = " If set, mutex statistics will be skipped."] # [doc = ""] # [doc = " This corresponds to the `x` character."] pub skip_mutex_statistics : bool , _p : () , }
};
}
