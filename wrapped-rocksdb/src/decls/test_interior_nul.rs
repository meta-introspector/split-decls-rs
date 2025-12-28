macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! test_interior_nul {
    () => {
        deps!();
        # [test] # [should_panic (expected = "input contained interior nul byte")] fn test_interior_nul () { PropName :: new_unwrap ("interior nul\0\0") ; }
    };
}

test_interior_nul!()