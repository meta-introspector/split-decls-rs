macro_rules! imp {
    () => {
        # [path = "wrapper.rs"] # [cfg (wrap_proc_macro)] mod imp ;
    };
}

imp!();