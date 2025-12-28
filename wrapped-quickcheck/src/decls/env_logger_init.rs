macro_rules! env_logger_init {
    () => {
        # [cfg (not (feature = "use_logging"))] fn env_logger_init () { }
    };
}

env_logger_init!();