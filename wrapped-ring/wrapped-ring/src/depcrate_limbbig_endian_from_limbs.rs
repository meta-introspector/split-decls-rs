// Generated macro for big_endian_from_limbs (function)
macro_rules! Depcrate_limbbig_endian_from_limbs {
() => {
// Module: crate::limb
// Provides: {"big_endian_from_limbs"}
// Dependencies: {}
pub fn big_endian_from_limbs (limbs : & [Limb] , out : & mut [u8]) { let be_bytes = unstripped_be_bytes (limbs) ; assert_eq ! (out . len () , be_bytes . len ()) ; out . iter_mut () . zip (be_bytes) . for_each (| (o , i) | { * o = i ; }) ; }
};
}
