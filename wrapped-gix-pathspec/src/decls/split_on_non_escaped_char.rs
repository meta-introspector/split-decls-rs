macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! split_on_non_escaped_char {
    () => {
        deps!();
        fn split_on_non_escaped_char (input : & [u8] , split_char : u8 , mut f : impl FnMut (& [u8]) -> Result < () , Error > ,) -> Result < () , Error > { let mut i = 0 ; let mut last = 0 ; for window in input . windows (2) { i += 1 ; if window [0] != b'\\' && window [1] == split_char { let keyword = & input [last .. i] ; f (keyword) ? ; last = i + 1 ; } } let last_keyword = & input [last ..] ; f (last_keyword) }
    };
}

split_on_non_escaped_char!()