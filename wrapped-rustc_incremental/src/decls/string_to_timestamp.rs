macro_rules! deps {
    () => {
        Ok!();
    };
}

macro_rules! string_to_timestamp {
    () => {
        deps!();
        fn string_to_timestamp (s : & str) -> Result < SystemTime , & 'static str > { let micros_since_unix_epoch = match u64 :: from_str_radix (s , INT_ENCODE_BASE as u32) { Ok (micros) => micros , Err (_) => return Err ("timestamp not an int") , } ; let duration = Duration :: from_micros (micros_since_unix_epoch) ; Ok (UNIX_EPOCH + duration) }
    };
}

string_to_timestamp!()