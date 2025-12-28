macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! trusted_header_signature {
    () => {
        deps!();
        pub (crate) fn trusted_header_signature (name : & [u8] , value : & gix_actor :: SignatureRef < '_ > , out : & mut dyn io :: Write ,) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; value . write_to (out) ? ; out . write_all (NL) }
    };
}

trusted_header_signature!()