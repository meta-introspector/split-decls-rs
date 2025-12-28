macro_rules! getauxval_hwcap {
    () => {
        # [doc = " Linux helper function for calling `getauxval` to get `AT_HWCAP`."] # [cfg (target_os = "linux")] pub fn getauxval_hwcap () -> u64 { unsafe { libc :: getauxval (libc :: AT_HWCAP) } }
    };
}

getauxval_hwcap!();