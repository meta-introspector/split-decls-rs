macro_rules! impl_backtrace {
    () => {
        # [cfg (all (not (std_backtrace) , feature = "backtrace"))] macro_rules ! impl_backtrace { () => { impl core :: fmt :: Debug + core :: fmt :: Display } ; }
    };
}

impl_backtrace!();