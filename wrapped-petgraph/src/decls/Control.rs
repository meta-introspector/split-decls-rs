macro_rules! deps {
    () => {
        DfsEvent!();
    };
}

macro_rules! Control {
    () => {
        deps!();
        # [doc = " Control flow for `depth_first_search` callbacks."] # [derive (Copy , Clone , Debug)] pub enum Control < B > { # [doc = " Continue the DFS traversal as normal."] Continue , # [doc = " Prune the current node from the DFS traversal. No more edges from this"] # [doc = " node will be reported to the callback. A `DfsEvent::Finish` for this"] # [doc = " node will still be reported. This can be returned in response to any"] # [doc = " `DfsEvent`, except `Finish`, which will panic."] Prune , # [doc = " Stop the DFS traversal and return the provided value."] Break (B) , }
    };
}

Control!();