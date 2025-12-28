macro_rules! deps {
    () => {
        ModuleCodegen!();
    };
}

macro_rules! temp_stable_hash_impls {
    () => {
        deps!();
        mod temp_stable_hash_impls { use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ; use crate :: ModuleCodegen ; impl < HCX , M > HashStable < HCX > for ModuleCodegen < M > { fn hash_stable (& self , _ : & mut HCX , _ : & mut StableHasher) { } } }
    };
}

temp_stable_hash_impls!()