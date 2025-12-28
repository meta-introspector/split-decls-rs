macro_rules! macro_7 {
    () => {
        # [cfg (feature = "logging")] thread_local ! { static LOG_DEPTH : RefCell < usize > = RefCell :: new (0) ; }
    };
}

macro_7!()