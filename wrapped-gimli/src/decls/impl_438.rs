macro_rules! deps {
    () => {
        ListsHeader!();
        Encoding!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl ListsHeader { # [doc = " Return the serialized size of the table header."] # [allow (dead_code)] # [inline] fn size (self) -> u8 { ListsHeader :: size_for_encoding (self . encoding) } # [doc = " Return the serialized size of the table header."] # [inline] pub (crate) fn size_for_encoding (encoding : Encoding) -> u8 { encoding . format . initial_length_size () + 2 + 1 + 1 + 4 } }
    };
}

impl_438!()