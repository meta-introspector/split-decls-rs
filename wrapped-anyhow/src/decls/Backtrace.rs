macro_rules! Backtrace {
    () => {
        # [cfg (not (any (std_backtrace , feature = "backtrace")))] pub (crate) enum Backtrace { }
    };
}

Backtrace!()