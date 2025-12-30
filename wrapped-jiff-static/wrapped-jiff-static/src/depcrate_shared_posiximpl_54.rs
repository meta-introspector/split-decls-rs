// Generated macro for impl_54 (impl)
macro_rules! Depcrate_shared_posiximpl_54 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " Helper routines for parsing a POSIX `TZ` string."] impl < 's > Parser < 's > { # [doc = " Bump the parser to the next byte."] # [doc = ""] # [doc = " If the end of the input has been reached, then `false` is returned."] fn bump (& self) -> bool { if self . is_done () { return false ; } self . pos . set (self . pos () . checked_add (1) . expect ("pos cannot overflow usize") ,) ; ! self . is_done () } # [doc = " Returns true if the next call to `bump` would return false."] fn is_done (& self) -> bool { self . pos () == self . tz . len () } # [doc = " Return the byte at the current position of the parser."] # [doc = ""] # [doc = " This panics if the parser is positioned at the end of the TZ"] # [doc = " string."] fn byte (& self) -> u8 { self . tz [self . pos ()] } # [doc = " Return the byte at the current position of the parser. If the TZ"] # [doc = " string has been exhausted, then this returns `None`."] fn maybe_byte (& self) -> Option < u8 > { self . tz . get (self . pos ()) . copied () } # [doc = " Return the current byte offset of the parser."] # [doc = ""] # [doc = " The offset starts at `0` from the beginning of the TZ string."] fn pos (& self) -> usize { self . pos . get () } # [doc = " Returns the remaining bytes of the TZ string."] # [doc = ""] # [doc = " This includes `self.byte()`. It may be empty."] fn remaining (& self) -> & 's [u8] { & self . tz [self . pos () ..] } }
};
}
