// Generated macro for _num_bigint (module)
macro_rules! Depcrate_hash_num_bigint {
() => {
// Module: crate::hash
// Provides: {"_num_bigint"}
// Dependencies: {}
# [cfg (feature = "num-bigint")] mod _num_bigint { use super :: * ; use num_bigint :: { BigInt , BigUint } ; use num_traits :: ToPrimitive ; impl NumHash for BigUint { fn num_hash < H : Hasher > (& self , state : & mut H) { (self % BigUint :: from (M127U)) . to_i128 () . unwrap () . hash (state) } } impl NumHash for BigInt { fn num_hash < H : Hasher > (& self , state : & mut H) { (self % BigInt :: from (M127)) . to_i128 () . unwrap () . hash (state) } } }
};
}
