// Generated macro for impl_925 (impl)
macro_rules! Depcrate_packetimpl_925 {
() => {
// Module: crate::packet
// Provides: {"impl_925"}
// Dependencies: {}
impl PktNumWindow { pub fn insert (& mut self , seq : u64) { if seq < self . lower { return ; } if seq > self . upper () { let diff = seq - self . upper () ; self . lower += diff ; self . window = self . window . checked_shl (diff as u32) . unwrap_or (0) ; } let mask = 1_u128 << (self . upper () - seq) ; self . window |= mask ; } pub fn contains (& mut self , seq : u64) -> bool { if seq > self . upper () { return false ; } if seq < self . lower { return true ; } let mask = 1_u128 << (self . upper () - seq) ; self . window & mask != 0 } fn upper (& self) -> u64 { self . lower . saturating_add (size_of :: < u128 > () as u64 * 8) - 1 } }
};
}
