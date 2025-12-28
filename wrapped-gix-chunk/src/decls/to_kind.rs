macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! to_kind {
    () => {
        deps!();
        fn to_kind (data : & [u8]) -> crate :: Id { data [.. 4] . try_into () . unwrap () }
    };
}

to_kind!()