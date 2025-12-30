// Generated macro for impl_217 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_217 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf16CharIndices < 'a > { fn next_back (& mut self) -> Option < (usize , Utf16Char) > { if self . index < self . str . len () { let rev = self . str . bytes () . rev () ; let len = 1 + rev . take_while (| b | b & 0b1100_0000 == 0b1000_0000) . count () ; let starts = self . str . len () - len ; let (u16c , _) = Utf16Char :: from_str_start (& self . str [starts ..]) . unwrap () ; self . str = & self . str [.. starts] ; Some ((starts , u16c)) } else { None } } }
};
}
