macro_rules! Extended {
    () => {
        # [cfg (not (target_pointer_width = "16"))] # [allow (dead_code)] # [repr (C)] struct Extended < T : Copy , const N : usize > { # [cfg (target_endian = "big")] pad : [MaybeUninit < T > ; N] , v : MaybeUninit < T > , # [cfg (target_endian = "little")] pad : [MaybeUninit < T > ; N] , }
    };
}

Extended!()