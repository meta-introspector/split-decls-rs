// Generated macro for impl_688 (impl)
macro_rules! Depcrate_util_alphabetimpl_688 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_688"}
// Dependencies: {}
impl < 'a > Iterator for ByteClassRepresentatives < 'a > { type Item = Unit ; fn next (& mut self) -> Option < Unit > { while self . cur_byte < self . end_byte . unwrap_or (256) { let byte = u8 :: try_from (self . cur_byte) . unwrap () ; let class = self . classes . get (byte) ; self . cur_byte += 1 ; if self . last_class != Some (class) { self . last_class = Some (class) ; return Some (Unit :: u8 (byte)) ; } } if self . cur_byte != usize :: MAX && self . end_byte . is_none () { self . cur_byte = usize :: MAX ; return Some (self . classes . eoi ()) ; } None } }
};
}
