macro_rules! deps {
    () => {
        ToHex!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > + ? Sized > ToHex for T { fn encode_hex < U : iter :: FromIterator < char > > (& self) -> U { encode_to_iter (HEX_CHARS_LOWER , self . as_ref ()) } fn encode_hex_upper < U : iter :: FromIterator < char > > (& self) -> U { encode_to_iter (HEX_CHARS_UPPER , self . as_ref ()) } }
    };
}

impl_18!()