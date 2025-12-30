// Generated macro for TzifTransitions (struct)
macro_rules! Depcrate_sharedTzifTransitions {
() => {
// Module: crate::shared
// Provides: {"TzifTransitions"}
// Dependencies: {}
# [doc = " The set of transitions in TZif data, laid out in column orientation."] # [doc = ""] # [doc = " The column orientation is used to make TZ lookups faster. Specifically,"] # [doc = " for finding an offset for a timestamp, we do a binary search on"] # [doc = " `timestamps`. For finding an offset for a local datetime, we do a binary"] # [doc = " search on `civil_starts`. By making these two distinct sequences with"] # [doc = " nothing else in them, we make them as small as possible and thus improve"] # [doc = " cache locality."] # [doc = ""] # [doc = " All sequences in this type are in correspondence with one another. They"] # [doc = " are all guaranteed to have the same length."] # [derive (Clone , Debug)] pub struct TzifTransitions < TIMESTAMPS , STARTS , ENDS , INFOS > { # [doc = " The timestamp at which this transition begins."] pub timestamps : TIMESTAMPS , # [doc = " The wall clock time for when a transition begins."] pub civil_starts : STARTS , # [doc = " The wall clock time for when a transition ends."] # [doc = ""] # [doc = " This is only non-zero when the transition kind is a gap or a fold."] pub civil_ends : ENDS , # [doc = " Any other relevant data about a transition, such as its local type"] # [doc = " index and the transition kind."] pub infos : INFOS , }
};
}
