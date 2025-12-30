// Generated macro for Node (struct)
macro_rules! Depcrate_debt_listNode {
() => {
// Module: crate::debt::list
// Provides: {"Node"}
// Dependencies: {}
# [doc = " One thread-local node for debts."] # [repr (C , align (64))] pub (crate) struct Node { fast : FastSlots , helping : HelpingSlots , in_use : AtomicUsize , next : * const Node , active_writers : AtomicUsize , }
};
}
