macro_rules! INVALID_STRING_ID {
    () => {
        # [doc = " Some random string ID that we make sure cannot be generated or assigned to."] const INVALID_STRING_ID : u64 = METADATA_STRING_ID + 1 ;
    };
}

INVALID_STRING_ID!()