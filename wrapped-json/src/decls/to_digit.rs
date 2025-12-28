macro_rules! to_digit {
    () => {
        # [inline] pub (crate) fn to_digit (c : u8) -> Option < u32 > { (c as char) . to_digit (10) }
    };
}

to_digit!();