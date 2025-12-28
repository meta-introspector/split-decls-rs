macro_rules! is_bin_digit {
    () => {
        # [doc = " Tests if byte is ASCII binary digit: 0-1"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::character::is_bin_digit;"] # [doc = " assert_eq!(is_bin_digit(b'a'), false);"] # [doc = " assert_eq!(is_bin_digit(b'2'), false);"] # [doc = " assert_eq!(is_bin_digit(b'0'), true);"] # [doc = " assert_eq!(is_bin_digit(b'1'), true);"] # [doc = " ```"] # [inline] pub fn is_bin_digit (chr : u8) -> bool { matches ! (chr , 0x30 ..= 0x31) }
    };
}

is_bin_digit!();