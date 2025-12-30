// Generated macro for impl_301 (impl)
macro_rules! Depcrate_common_cache_controlimpl_301 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_301"}
// Dependencies: {}
impl Flags { const NO_CACHE : Self = Self { bits : 0b000000001 } ; const NO_STORE : Self = Self { bits : 0b000000010 } ; const NO_TRANSFORM : Self = Self { bits : 0b000000100 } ; const ONLY_IF_CACHED : Self = Self { bits : 0b000001000 } ; const MUST_REVALIDATE : Self = Self { bits : 0b000010000 } ; const PUBLIC : Self = Self { bits : 0b000100000 } ; const PRIVATE : Self = Self { bits : 0b001000000 } ; const PROXY_REVALIDATE : Self = Self { bits : 0b010000000 } ; const IMMUTABLE : Self = Self { bits : 0b100000000 } ; const MUST_UNDERSTAND : Self = Self { bits : 0b1000000000 } ; fn empty () -> Self { Self { bits : 0 } } fn contains (& self , flag : Self) -> bool { (self . bits & flag . bits) != 0 } fn insert (& mut self , flag : Self) { self . bits |= flag . bits ; } }
};
}
