macro_rules! LineInfo {
    () => {
        # [derive (Debug)] pub (crate) struct LineInfo < 'a > { pub (crate) line : & 'a str , pub (crate) line_index : usize , pub (crate) start_byte : usize , pub (crate) end_byte : usize , end_line_size : usize , }
    };
}

LineInfo!();