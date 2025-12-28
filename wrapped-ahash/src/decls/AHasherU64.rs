macro_rules! AHasherU64 {
    () => {
        # [cfg (specialize)] pub (crate) struct AHasherU64 { pub (crate) buffer : u64 , pub (crate) pad : u64 , }
    };
}

AHasherU64!()