macro_rules! zero_sha {
    () => {
        fn zero_sha () -> ObjectId { use std :: str :: FromStr ; ObjectId :: from_str ("0000000000000000000000000000000000000000") . unwrap () }
    };
}

zero_sha!()