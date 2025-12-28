macro_rules! deps {
    () => {
        StateID!();
        DeserializeError!();
    };
}

macro_rules! read_state_id {
    () => {
        deps!();
        # [doc = " Reads a state ID from the given slice. If the slice has insufficient"] # [doc = " length, then this panics. If the deserialized integer exceeds the state ID"] # [doc = " limit for the current target, then this returns an error."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn read_state_id (slice : & [u8] , what : & 'static str ,) -> Result < (StateID , usize) , DeserializeError > { let bytes : [u8 ; StateID :: SIZE] = slice [.. StateID :: SIZE] . try_into () . unwrap () ; let sid = StateID :: from_ne_bytes (bytes) . map_err (| err | DeserializeError :: state_id_error (err , what)) ? ; Ok ((sid , StateID :: SIZE)) }
    };
}

read_state_id!();