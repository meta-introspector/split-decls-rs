macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! trusted_header_field {
    () => {
        deps!();
        pub (crate) fn trusted_header_field (name : & [u8] , value : & [u8] , out : & mut dyn io :: Write) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; out . write_all (value) ? ; out . write_all (NL) }
    };
}

trusted_header_field!();