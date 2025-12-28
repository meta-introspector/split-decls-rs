macro_rules! deps {
    () => {
        StateID!();
        Endian!();
    };
}

macro_rules! write_state_id {
    () => {
        deps!();
        # [doc = " Write the given state ID to the beginning of the given slice of bytes"] # [doc = " using the specified endianness. The given slice must have length at least"] # [doc = " `StateID::SIZE`, or else this panics. Upon success, the total number of"] # [doc = " bytes written is returned."] pub (crate) fn write_state_id < E : Endian > (sid : StateID , dst : & mut [u8] ,) -> usize { E :: write_u32 (sid . as_u32 () , dst) ; StateID :: SIZE }
    };
}

write_state_id!();