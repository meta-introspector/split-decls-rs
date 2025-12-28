macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! read_state_id_unchecked {
    () => {
        deps!();
        # [doc = " Reads a state ID from the given slice. If the slice has insufficient"] # [doc = " length, then this panics. Otherwise, the deserialized integer is assumed"] # [doc = " to be a valid state ID."] # [doc = ""] # [doc = " This also returns the number of bytes read."] pub (crate) fn read_state_id_unchecked (slice : & [u8]) -> (StateID , usize) { let sid = StateID :: from_ne_bytes_unchecked (slice [.. StateID :: SIZE] . try_into () . unwrap () ,) ; (sid , StateID :: SIZE) }
    };
}

read_state_id_unchecked!()