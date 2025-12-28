macro_rules! State {
    () => {
        # [derive (Clone , Copy)] struct State { is_open : bool , num_messages : usize , }
    };
}

State!();