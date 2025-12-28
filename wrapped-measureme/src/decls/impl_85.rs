macro_rules! deps {
    () => {
        SerializableString!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl SerializableString for str { # [inline] fn serialized_size (& self) -> usize { self . len () + 1 } # [inline] fn serialize (& self , bytes : & mut [u8]) { let last_byte_index = bytes . len () - 1 ; bytes [0 .. last_byte_index] . copy_from_slice (self . as_bytes ()) ; bytes [last_byte_index] = TERMINATOR ; } }
    };
}

impl_85!();