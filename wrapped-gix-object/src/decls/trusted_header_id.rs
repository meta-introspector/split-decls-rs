macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! trusted_header_id {
    () => {
        deps!();
        pub (crate) fn trusted_header_id (name : & [u8] , value : & gix_hash :: ObjectId , mut out : & mut dyn io :: Write ,) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; value . write_hex_to (& mut out) ? ; out . write_all (NL) }
    };
}

trusted_header_id!()