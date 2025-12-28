macro_rules! deps {
    () => {
        BE!();
        Endian!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl Endian for BE { fn write_u16 (n : u16 , dst : & mut [u8]) { dst [.. 2] . copy_from_slice (& n . to_be_bytes ()) ; } fn write_u32 (n : u32 , dst : & mut [u8]) { dst [.. 4] . copy_from_slice (& n . to_be_bytes ()) ; } fn write_u128 (n : u128 , dst : & mut [u8]) { dst [.. 16] . copy_from_slice (& n . to_be_bytes ()) ; } }
    };
}

impl_849!()