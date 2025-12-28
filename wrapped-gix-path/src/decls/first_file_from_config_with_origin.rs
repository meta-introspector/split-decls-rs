macro_rules! first_file_from_config_with_origin {
    () => {
        fn first_file_from_config_with_origin (source : & BStr) -> Option < & BStr > { let file = source . strip_prefix (b"file:") ? ; let end_pos = file . find_byte (b'\0') ? ; file [.. end_pos] . as_bstr () . into () }
    };
}

first_file_from_config_with_origin!()