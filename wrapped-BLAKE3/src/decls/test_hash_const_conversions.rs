macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! test_hash_const_conversions {
    () => {
        deps!();
        # [test] const fn test_hash_const_conversions () { let bytes = [42 ; 32] ; let hash = crate :: Hash :: from_bytes (bytes) ; _ = hash . as_bytes () ; }
    };
}

test_hash_const_conversions!();