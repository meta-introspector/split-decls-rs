macro_rules! deps {
    () => {
        Endian!();
        SerializeError!();
    };
}

macro_rules! write_endianness_check {
    () => {
        deps!();
        # [doc = " Writes 0xFEFF as an integer using the given endianness."] # [doc = ""] # [doc = " This is useful for writing into the header of a serialized object. It can"] # [doc = " be read during deserialization as a sanity check to ensure the proper"] # [doc = " endianness is used."] # [doc = ""] # [doc = " Upon success, the total number of bytes written is returned."] pub (crate) fn write_endianness_check < E : Endian > (dst : & mut [u8] ,) -> Result < usize , SerializeError > { let nwrite = write_endianness_check_len () ; if dst . len () < nwrite { return Err (SerializeError :: buffer_too_small ("endianness check")) ; } E :: write_u32 (0xFEFF , dst) ; Ok (nwrite) }
    };
}

write_endianness_check!();