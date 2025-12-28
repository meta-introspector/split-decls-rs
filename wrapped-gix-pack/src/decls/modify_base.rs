macro_rules! deps {
    () => {
        TreeEntry!();
        Kind!();
        Error!();
        Entry!();
    };
}

macro_rules! modify_base {
    () => {
        deps!();
        fn modify_base (entry : & mut TreeEntry , pack_entry : & crate :: data :: Entry , decompressed : & [u8] , hash : gix_hash :: Kind ,) -> Result < () , gix_hash :: hasher :: Error > { let object_kind = pack_entry . header . as_kind () . expect ("base object as source of iteration") ; let id = gix_object :: compute_hash (hash , object_kind , decompressed) ? ; entry . id = id ; Ok (()) }
    };
}

modify_base!()