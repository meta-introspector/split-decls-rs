macro_rules! State {
    () => {
        # [derive (Clone)] enum State { Baseline (baseline :: State) , Specialized (specialized :: State) , }
    };
}

State!();