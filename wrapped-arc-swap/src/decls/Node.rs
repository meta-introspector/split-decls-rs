macro_rules! Node {
    () => {
        # [doc = " One thread-local node for debts."] # [repr (C , align (64))] pub (crate) struct Node { fast : FastSlots , helping : HelpingSlots , in_use : AtomicUsize , next : * const Node , active_writers : AtomicUsize , }
    };
}

Node!();