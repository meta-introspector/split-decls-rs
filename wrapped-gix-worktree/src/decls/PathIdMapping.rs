macro_rules! PathIdMapping {
    () => {
        pub (crate) type PathIdMapping = (BString , gix_hash :: ObjectId) ;
    };
}

PathIdMapping!();