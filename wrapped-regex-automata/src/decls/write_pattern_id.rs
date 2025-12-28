macro_rules! deps {
    () => {
        Endian!();
        PatternID!();
    };
}

macro_rules! write_pattern_id {
    () => {
        deps!();
        # [doc = " Write the given pattern ID to the beginning of the given slice of bytes"] # [doc = " using the specified endianness. The given slice must have length at least"] # [doc = " `PatternID::SIZE`, or else this panics. Upon success, the total number of"] # [doc = " bytes written is returned."] pub (crate) fn write_pattern_id < E : Endian > (pid : PatternID , dst : & mut [u8] ,) -> usize { E :: write_u32 (pid . as_u32 () , dst) ; PatternID :: SIZE }
    };
}

write_pattern_id!()