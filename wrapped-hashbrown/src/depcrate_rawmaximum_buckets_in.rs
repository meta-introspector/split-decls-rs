// Generated macro for maximum_buckets_in (function)
macro_rules! Depcrate_rawmaximum_buckets_in {
() => {
// Module: crate::raw
// Provides: {"maximum_buckets_in"}
// Dependencies: {}
# [doc = " Finds the largest number of buckets that can fit in `allocation_size`"] # [doc = " provided the given TableLayout."] # [doc = ""] # [doc = " This relies on some invariants of `capacity_to_buckets`, so only feed in"] # [doc = " an `allocation_size` calculated from `capacity_to_buckets`."] fn maximum_buckets_in (allocation_size : usize , table_layout : TableLayout , group_width : usize ,) -> usize { let x = (allocation_size - group_width) / (table_layout . size + 1) ; prev_pow2 (x) }
};
}
