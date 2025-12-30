// Generated macro for DecapsulationKey (struct)
macro_rules! Depcrate_hazardous_kem_xwingDecapsulationKey {
() => {
// Module: crate::hazardous::kem::xwing
// Provides: {"DecapsulationKey"}
// Dependencies: {}
# [derive (Debug , PartialEq)] # [doc = " A type to represent the `DecapsulationKey` that X-Wing produces."] # [doc = " This type's foremost responsibility is to cache key-expansions,"] # [doc = " to be re-used across multiple decapsulations with a single secret."] # [doc = ""] # [doc = " Calling [DecapsulationKey::unprotected_as_bytes] is equivalent to"] # [doc = " calling [Seed::unprotected_as_bytes]."] pub struct DecapsulationKey { seed : Seed , kp_m : mlkem768 :: KeyPair , sk_x : x25519 :: PrivateKey , pk_x : x25519 :: PublicKey , }
};
}
