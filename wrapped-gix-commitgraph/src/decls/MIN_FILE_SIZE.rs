macro_rules! MIN_FILE_SIZE {
    () => {
        const MIN_FILE_SIZE : usize = HEADER_LEN + gix_chunk :: file :: Index :: size_for_entries (3) + FAN_LEN * 4 + gix_hash :: Kind :: shortest () . len_in_bytes () ;
    };
}

MIN_FILE_SIZE!()