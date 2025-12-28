macro_rules! macro_0 {
    () => {
        # [cfg (all (anyhow_nightly_testing , feature = "std" , not (error_generic_member_access)))] compile_error ! ("Build script probe failed to compile.") ;
    };
}

macro_0!()