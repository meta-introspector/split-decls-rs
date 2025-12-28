macro_rules! random_seed {
    () => {
        # [cfg (all (any (target_arch = "wasm32" , target_arch = "wasm64") , target_os = "unknown" , not (feature = "js")))] fn random_seed () -> Option < u64 > { None }
    };
}

random_seed!();