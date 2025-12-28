macro_rules! ERROR_WILDCARDS {
    () => {
        const ERROR_WILDCARDS : & str = "wildcards are either regular `*` or recursive `**`" ;
    };
}

ERROR_WILDCARDS!();