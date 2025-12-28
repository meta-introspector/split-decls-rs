macro_rules! one_sha {
    () => {
        fn one_sha () -> ObjectId { use std :: str :: FromStr ; ObjectId :: from_str ("1111111111111111111111111111111111111111") . unwrap () }
    };
}

one_sha!()