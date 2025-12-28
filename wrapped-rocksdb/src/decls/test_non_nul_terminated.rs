macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! test_non_nul_terminated {
    () => {
        deps!();
        # [test] # [should_panic (expected = "input was not nul-terminated")] fn test_non_nul_terminated () { PropName :: new_unwrap ("no nul terminator") ; }
    };
}

test_non_nul_terminated!();