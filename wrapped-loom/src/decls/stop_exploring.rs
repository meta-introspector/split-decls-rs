macro_rules! stop_exploring {
    () => {
        # [doc = " Tells loom to stop exploring possible concurrent executions starting at this"] # [doc = " point."] # [doc = ""] # [doc = " Exploration can be enabled again with `explore`."] pub fn stop_exploring () { execution (| execution | { execution . path . critical () ; }) }
    };
}

stop_exploring!();