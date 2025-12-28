macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! detect_line_ending_or_nl {
    () => {
        deps!();
        pub fn detect_line_ending_or_nl (hunks : & [Hunk] , input : & mut InternedInput < & [u8] > , current_tokens : & [Token] ,) -> & 'static BStr { detect_line_ending (hunks , input , current_tokens) . unwrap_or (b"\n" . into ()) }
    };
}

detect_line_ending_or_nl!()