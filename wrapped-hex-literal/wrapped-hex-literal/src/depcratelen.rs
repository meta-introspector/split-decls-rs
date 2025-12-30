// Generated macro for len (function)
macro_rules! Depcratelen {
() => {
// Module: crate
// Provides: {"len"}
// Dependencies: {}
# [doc = " Compute length of a byte array which will be decoded from the strings."] # [doc = ""] # [doc = " This function is an implementation detail and SHOULD NOT be called directly!"] # [doc (hidden)] pub const fn len (strings : & [& [u8]]) -> usize { let mut i = 0 ; let mut len = 0 ; while i < strings . len () { let mut pos = 0 ; while let Some ((_ , new_pos)) = next_byte (strings [i] , pos) { len += 1 ; pos = new_pos ; } i += 1 ; } len }
};
}
