macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! encode_state {
    () => {
        deps!();
        fn encode_state (state : & State) -> usize { let mut num = state . num_messages ; if state . is_open { num |= OPEN_MASK ; } num }
    };
}

encode_state!();