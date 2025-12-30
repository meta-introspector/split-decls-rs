// Generated macro for impl_74 (impl)
macro_rules! Depcrate_mersenneimpl_74 {
() => {
// Module: crate::mersenne
// Provides: {"impl_74"}
// Dependencies: {}
impl < const P : u8 , const K : umax > FixedMersenne < P , K > { const BITMASK : umax = (1 << P) - 1 ; pub const MODULUS : umax = (1 << P) - K ; const fn reduce_single (v : umax) -> umax { let mut lo = v & Self :: BITMASK ; let mut hi = v >> P ; while hi > 0 { let sum = if K == 1 { hi + lo } else { hi * K + lo } ; lo = sum & Self :: BITMASK ; hi = sum >> P ; } if lo >= Self :: MODULUS { lo - Self :: MODULUS } else { lo } } fn reduce_double (v : udouble) -> umax { let mut lo = v . lo & Self :: BITMASK ; let mut hi = v >> P ; while hi . hi > 0 { let sum = if K == 1 { hi + lo } else { hi * K + lo } ; lo = sum . lo & Self :: BITMASK ; hi = sum >> P ; } let mut hi = hi . lo ; while hi > 0 { let sum = if K == 1 { hi + lo } else { hi * K + lo } ; lo = sum & Self :: BITMASK ; hi = sum >> P ; } if lo >= Self :: MODULUS { lo - Self :: MODULUS } else { lo } } }
};
}
