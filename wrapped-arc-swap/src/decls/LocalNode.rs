macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! LocalNode {
    () => {
        deps!();
        # [doc = " A wrapper around a node pointer, to un-claim the node on thread shutdown."] pub (crate) struct LocalNode { # [doc = " Node for this thread, if any."] # [doc = ""] # [doc = " We don't necessarily have to own one, but if we don't, we'll get one before the first use."] node : Cell < Option < & 'static Node > > , # [doc = " Thread-local data for the fast slots."] fast : FastLocal , # [doc = " Thread local data for the helping strategy."] helping : HelpingLocal , }
    };
}

LocalNode!();