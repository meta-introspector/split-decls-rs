macro_rules! deps {
    () => {
        DeserializeError!();
        StateID!();
    };
}

macro_rules! try_read_state_id {
    () => {
        deps!();
        # [doc = " Attempts to read a state ID from the given slice. If the slice has an"] # [doc = " insufficient number of bytes or if the state ID exceeds the limit for"] # [doc = " the current target, then this returns an error."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn try_read_state_id (slice : & [u8] , what : & 'static str ,) -> Result < (StateID , usize) , DeserializeError > { if slice . len () < StateID :: SIZE { return Err (DeserializeError :: buffer_too_small (what)) ; } read_state_id (slice , what) }
    };
}

try_read_state_id!();