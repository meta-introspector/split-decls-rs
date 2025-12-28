macro_rules! imp {
    () => {
        # [cfg (not (feature = "std"))] # [path = "core.rs"] mod imp ;
    };
}

imp!()