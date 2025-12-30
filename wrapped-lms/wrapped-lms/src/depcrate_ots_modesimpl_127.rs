// Generated macro for impl_127 (impl)
macro_rules! Depcrate_ots_modesimpl_127 {
() => {
// Module: crate::ots::modes
// Provides: {"impl_127"}
// Dependencies: {}
# [doc = " because trait associated consts cannot be used as generic values, we work around this by passing in an additional"] # [doc = " type representing the array length P used for private keys, which gets checked via some static asserts"] # [doc = ""] # [doc = " NLen and N are calculated using the associated OutputSize of the given Digest, as specified by"] # [doc = " https://datatracker.ietf.org/doc/html/rfc8554#section-4.1"] impl < Hasher : Digest , const W : usize , PP : ArraySize , const TC : u32 > LmsOtsMode for LmsOtsModeInternal < Hasher , W , PP , TC > { type Hasher = Hasher ; type NLen = Hasher :: OutputSize ; type PLen = PP ; const N : usize = Hasher :: OutputSize :: USIZE ; const W : usize = W ; const U : usize = (8 * Self :: N) . div_ceil (W) ; const V : usize = ((((1 << W) - 1) * Self :: U) . ilog2 () as usize / W) + 1 ; const P : usize = Self :: U + Self :: V ; const LS : usize = 16 - Self :: V * W ; const SIG_LEN : usize = 4 + Self :: N * (Self :: P + 1) ; }
};
}
