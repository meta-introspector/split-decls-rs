// Generated macro for temp_stable_hash_impls (module)
macro_rules! Depcrate_commontemp_stable_hash_impls {
() => {
// Module: crate::common
// Provides: {"temp_stable_hash_impls"}
// Dependencies: {}
mod temp_stable_hash_impls { use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ; use crate :: ModuleCodegen ; impl < HCX , M > HashStable < HCX > for ModuleCodegen < M > { fn hash_stable (& self , _ : & mut HCX , _ : & mut StableHasher) { } } }
};
}
