macro_rules! macro_15 {
    () => {
        cfg_if ! { if # [cfg (feature = "rustc-dep-of-std")] { extern crate rustc_std_workspace_core as core ; } }
    };
}

macro_15!()