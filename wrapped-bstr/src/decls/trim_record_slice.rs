macro_rules! trim_record_slice {
    () => {
        fn trim_record_slice (mut record : & [u8] , terminator : u8) -> & [u8] { if record . last_byte () == Some (terminator) { record = & record [.. record . len () - 1] ; } record }
    };
}

trim_record_slice!()