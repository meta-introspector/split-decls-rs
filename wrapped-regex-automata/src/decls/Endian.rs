macro_rules! Endian {
    () => {
        # [doc = " A simple trait for writing code generic over endianness."] # [doc = ""] # [doc = " This is similar to what byteorder provides, but we only need a very small"] # [doc = " subset."] pub (crate) trait Endian { # [doc = " Writes a u16 to the given destination buffer in a particular"] # [doc = " endianness. If the destination buffer has a length smaller than 2, then"] # [doc = " this panics."] fn write_u16 (n : u16 , dst : & mut [u8]) ; # [doc = " Writes a u32 to the given destination buffer in a particular"] # [doc = " endianness. If the destination buffer has a length smaller than 4, then"] # [doc = " this panics."] fn write_u32 (n : u32 , dst : & mut [u8]) ; # [doc = " Writes a u128 to the given destination buffer in a particular"] # [doc = " endianness. If the destination buffer has a length smaller than 16,"] # [doc = " then this panics."] fn write_u128 (n : u128 , dst : & mut [u8]) ; }
    };
}

Endian!()