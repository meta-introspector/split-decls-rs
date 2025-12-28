macro_rules! deps {
    () => {
        Sink!();
        StringSink!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (any (feature = "alloc" , test))] impl < 'a > Sink for StringSink < 'a > { type Error = () ; fn write_encoded_bytes (& mut self , s : & [u8]) -> Result < () , Self :: Error > { self . string . push_str (str :: from_utf8 (s) . unwrap ()) ; Ok (()) } }
    };
}

impl_6!()