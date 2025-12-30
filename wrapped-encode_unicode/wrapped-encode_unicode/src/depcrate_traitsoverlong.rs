// Generated macro for overlong (function)
macro_rules! Depcrate_traitsoverlong {
() => {
// Module: crate::traits
// Provides: {"overlong"}
// Dependencies: {}
fn overlong (first : u8 , second : u8) -> bool { if first < 0x80 { false } else if (first & 0xe0) == 0xc0 { (first & 0xfe) == 0xc0 } else if (first & 0xf0) == 0xe0 { first == 0xe0 && (second & 0xe0) == 0x80 } else { first == 0xf0 && (second & 0xf0) == 0x80 } }
};
}
