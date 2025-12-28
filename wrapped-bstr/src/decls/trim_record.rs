macro_rules! trim_record {
    () => {
        fn trim_record (record : & mut Vec < u8 > , terminator : u8) { if record . last_byte () == Some (terminator) { record . pop_byte () ; } }
    };
}

trim_record!()