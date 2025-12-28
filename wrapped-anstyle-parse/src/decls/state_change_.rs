macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! state_change_ {
    () => {
        deps!();
        # [inline] const fn state_change_ (state : State , byte : u8) -> u8 { let state_idx = state as usize ; let byte_idx = byte as usize ; table :: STATE_CHANGES [state_idx] [byte_idx] }
    };
}

state_change_!();