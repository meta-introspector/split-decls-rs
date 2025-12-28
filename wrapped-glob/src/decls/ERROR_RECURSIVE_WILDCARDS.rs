macro_rules! ERROR_RECURSIVE_WILDCARDS {
    () => {
        const ERROR_RECURSIVE_WILDCARDS : & str = "recursive wildcards must form a single path \
                                         component" ;
    };
}

ERROR_RECURSIVE_WILDCARDS!();