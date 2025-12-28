macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl Error { # [doc = " The one-indexed line number where the error occurred. This is determined"] # [doc = " by the number of newlines that were successfully parsed."] # [must_use] pub const fn line_number (& self) -> usize { self . line_number + 1 } # [doc = " The data that was left unparsed, which contains the cause of the parse error."] # [must_use] pub fn remaining_data (& self) -> & [u8] { & self . parsed_until } }
    };
}

impl_153!();