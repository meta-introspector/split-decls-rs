macro_rules! mpmc {
    () => {
        # [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] pub mod mpmc ;
    };
}

mpmc!();