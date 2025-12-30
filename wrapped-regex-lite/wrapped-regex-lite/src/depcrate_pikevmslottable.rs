// Generated macro for SlotTable (struct)
macro_rules! Depcrate_pikevmSlotTable {
() => {
// Module: crate::pikevm
// Provides: {"SlotTable"}
// Dependencies: {}
# [doc = " A table of slots, where each row represent a state in an NFA. Thus, the"] # [doc = " table has room for storing slots for every single state in an NFA."] # [doc = ""] # [doc = " This table is represented with a single contiguous allocation. In general,"] # [doc = " the notion of \"capturing group\" doesn't really exist at this level of"] # [doc = " abstraction, hence the name \"slot\" instead. (Indeed, every capturing group"] # [doc = " maps to a pair of slots, one for the start offset and one for the end"] # [doc = " offset.) Slots are indexed by the `Captures` NFA state."] # [derive (Clone , Debug)] struct SlotTable { # [doc = " The actual table of offsets."] table : Vec < Option < NonMaxUsize > > , # [doc = " The number of slots per state, i.e., the table's stride or the length"] # [doc = " of each row."] slots_per_state : usize , # [doc = " The number of slots in the caller-provided `Captures` value for the"] # [doc = " current search. Setting this to `slots_per_state` is always correct,"] # [doc = " but may be wasteful."] slots_for_captures : usize , }
};
}
