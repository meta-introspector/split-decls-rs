macro_rules! macro_685 {
    () => {
        # [cfg (all (feature = "loadable_extension" , feature = "load_extension"))] compile_error ! ("feature \"loadable_extension\" and feature \"load_extension\" cannot be enabled at the same time") ;
    };
}

macro_685!()