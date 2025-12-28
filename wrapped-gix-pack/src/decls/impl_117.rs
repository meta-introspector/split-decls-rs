macro_rules! deps {
    () => {
        Location!();
        Offset!();
        EntryRange!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Location { # [doc = " Compute a range suitable for lookup in pack data using the [`entry_slice()`][crate::data::File::entry_slice()] method."] pub fn entry_range (& self , pack_offset : data :: Offset) -> crate :: data :: EntryRange { pack_offset .. pack_offset + self . entry_size as u64 } }
    };
}

impl_117!()