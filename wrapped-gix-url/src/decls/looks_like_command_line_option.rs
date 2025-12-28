macro_rules! looks_like_command_line_option {
    () => {
        fn looks_like_command_line_option (b : & [u8]) -> bool { b . first () == Some (& b'-') }
    };
}

looks_like_command_line_option!()