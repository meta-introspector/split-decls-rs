macro_rules! Pair {
    () => {
        # [allow (dead_code)] # [derive (Clone , Copy)] # [repr (C)] pub (crate) struct Pair < T : Copy > { # [cfg (any (target_endian = "little" , target_arch = "aarch64" , target_arch = "arm" , target_arch = "arm64ec" ,))] pub (crate) lo : MaybeUninit < T > , pub (crate) hi : MaybeUninit < T > , # [cfg (not (any (target_endian = "little" , target_arch = "aarch64" , target_arch = "arm" , target_arch = "arm64ec" ,)))] pub (crate) lo : MaybeUninit < T > , }
    };
}

Pair!();