macro_rules! integer {
    () => {
        fn integer (b : & [u32] , r : usize) -> u64 { let j = (2 * r - 1) * 16 ; u64 :: from (b [j]) | u64 :: from (b [j + 1]) << 32 }
    };
}

integer!();