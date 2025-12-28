macro_rules! test_into {
    () => {
        fn test_into < T , U > (orig : T) where T : Into < U > , { let _ = orig . into () ; }
    };
}

test_into!()