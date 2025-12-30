// Generated macro for impl_133 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_133 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf8CharIndices < 'a > { fn next_back (& mut self) -> Option < (usize , Utf8Char) > { if self . index < self . str . len () { let rev = self . str . bytes () . rev () ; let len = 1 + rev . take_while (| b | b & 0b1100_0000 == 0b1000_0000) . count () ; let starts = self . str . len () - len ; let (u8c , _) = Utf8Char :: from_str_start (& self . str [starts ..]) . unwrap () ; self . str = & self . str [.. starts] ; Some ((starts , u8c)) } else { None } } }
};
}
