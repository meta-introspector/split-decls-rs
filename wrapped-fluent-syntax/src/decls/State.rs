macro_rules! State {
    () => {
        # [derive (Debug , Default , PartialEq)] struct State { wrote_non_junk_entry : bool , }
    };
}

State!();