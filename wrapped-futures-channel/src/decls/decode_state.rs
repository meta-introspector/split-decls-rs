macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! decode_state {
    () => {
        deps!();
        fn decode_state (num : usize) -> State { State { is_open : num & OPEN_MASK == OPEN_MASK , num_messages : num & MAX_CAPACITY } }
    };
}

decode_state!()