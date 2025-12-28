macro_rules! lazy {
    () => {
        # [cfg_attr (feature = "spin_no_std" , path = "core_lazy.rs")] # [cfg_attr (not (feature = "spin_no_std") , path = "inline_lazy.rs")] # [doc (hidden)] pub mod lazy ;
    };
}

lazy!()