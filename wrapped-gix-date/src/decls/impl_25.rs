macro_rules! deps {
    () => {
        TimeBuf!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl TimeBuf { # [doc = " Represent this instance as standard string, serialized in a format compatible with"] # [doc = " signature fields in Git commits, also known as anything parseable as [raw format](function::parse_header())."] pub fn as_str (& self) -> & str { let time_bytes = self . buf . as_slice () ; # [allow (unsafe_code)] unsafe { std :: str :: from_utf8_unchecked (time_bytes) } } # [doc = " Clear the previous content."] fn clear (& mut self) { self . buf . clear () ; } }
    };
}

impl_25!();