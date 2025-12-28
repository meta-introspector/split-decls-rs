macro_rules! Vectorization {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub (crate) enum Vectorization { None = 0 , # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] SSE41 = 1 , # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] AVX2 = 2 , # [cfg (target_arch = "aarch64")] Neon = 3 , }
    };
}

Vectorization!();