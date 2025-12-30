// Generated macro for GVNContext (struct)
macro_rules! Depcrate_egraphGVNContext {
() => {
// Module: crate::egraph
// Provides: {"GVNContext"}
// Dependencies: {}
# [doc = " Implementation of external-context equality and hashing on"] # [doc = " InstructionData. This allows us to deduplicate instructions given"] # [doc = " some context that lets us see its value lists, so we don't need to"] # [doc = " store arguments inline in the `InstuctionData` (or alongside it in"] # [doc = " some newly-defined key type) in all cases."] struct GVNContext < 'a > { value_lists : & 'a ValueListPool , }
};
}
