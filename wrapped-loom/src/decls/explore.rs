macro_rules! explore {
    () => {
        # [doc = " Tells loom to explore possible concurrent executions starting at this point."] pub fn explore () { execution (| execution | { execution . path . explore_state () ; }) }
    };
}

explore!()