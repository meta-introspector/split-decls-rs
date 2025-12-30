// Generated macro for impl_687 (impl)
macro_rules! Depcrate_util_alphabetimpl_687 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_687"}
// Dependencies: {}
impl < 'a > Iterator for ByteClassElements < 'a > { type Item = Unit ; fn next (& mut self) -> Option < Unit > { while self . byte < 256 { let byte = u8 :: try_from (self . byte) . unwrap () ; self . byte += 1 ; if self . class . is_byte (self . classes . get (byte)) { return Some (Unit :: u8 (byte)) ; } } if self . byte < 257 { self . byte += 1 ; if self . class . is_eoi () { return Some (Unit :: eoi (256)) ; } } None } }
};
}
