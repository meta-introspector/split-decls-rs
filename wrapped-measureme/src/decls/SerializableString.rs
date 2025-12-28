macro_rules! SerializableString {
    () => {
        # [doc = " Anything that implements `SerializableString` can be written to a"] # [doc = " `StringTable`."] pub trait SerializableString { fn serialized_size (& self) -> usize ; fn serialize (& self , bytes : & mut [u8]) ; }
    };
}

SerializableString!();