macro_rules! backtrace_if_absent {
    () => {
        # [cfg (all (any (feature = "std" , not (anyhow_no_core_error)) , not (std_backtrace) , not (feature = "backtrace") ,))] macro_rules ! backtrace_if_absent { ($ err : expr) => { None } ; }
    };
}

backtrace_if_absent!()