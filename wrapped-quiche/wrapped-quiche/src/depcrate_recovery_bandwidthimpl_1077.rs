// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_recovery_bandwidthimpl_1077 {
() => {
// Module: crate::recovery::bandwidth
// Provides: {"impl_1077"}
// Dependencies: {}
impl Bandwidth { pub const fn from_bytes_and_time_delta (bytes : usize , time_delta : Duration ,) -> Self { if bytes == 0 { return Bandwidth { bits_per_second : 0 } ; } let mut nanos = time_delta . as_nanos () as u64 ; if nanos == 0 { nanos = 1 ; } let num_nano_bits = 8 * bytes as u64 * NUM_NANOS_PER_SECOND ; if num_nano_bits < nanos { return Bandwidth { bits_per_second : 1 } ; } Bandwidth { bits_per_second : num_nano_bits / nanos , } } # [allow (dead_code)] pub const fn from_bytes_per_second (bytes_per_second : u64) -> Self { Bandwidth { bits_per_second : bytes_per_second * 8 , } } # [allow (dead_code)] pub const fn to_bits_per_second (self) -> u64 { self . bits_per_second } pub const fn to_bytes_per_second (self) -> u64 { self . bits_per_second / 8 } pub const fn from_kbits_per_second (k_bits_per_second : u64) -> Self { Bandwidth { bits_per_second : k_bits_per_second * 1_000 , } } # [allow (dead_code)] pub const fn from_mbits_per_second (m_bits_per_second : u64) -> Self { Bandwidth :: from_kbits_per_second (m_bits_per_second * 1_000) } pub const fn infinite () -> Self { Bandwidth { bits_per_second : u64 :: MAX , } } pub const fn zero () -> Self { Bandwidth { bits_per_second : 0 } } pub fn transfer_time (& self , bytes : usize) -> Duration { if self . bits_per_second == 0 { Duration :: ZERO } else { Duration :: from_nanos ((bytes as u64 * 8 * NUM_NANOS_PER_SECOND) / self . bits_per_second ,) } } pub fn to_bytes_per_period (self , time_period : Duration) -> u64 { self . bits_per_second * time_period . as_nanos () as u64 / 8 / NUM_NANOS_PER_SECOND } }
};
}
