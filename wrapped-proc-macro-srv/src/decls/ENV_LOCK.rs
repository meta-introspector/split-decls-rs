macro_rules! ENV_LOCK {
    () => {
        static ENV_LOCK : std :: sync :: Mutex < () > = std :: sync :: Mutex :: new (()) ;
    };
}

ENV_LOCK!();