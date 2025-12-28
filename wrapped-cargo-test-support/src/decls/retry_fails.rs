macro_rules! retry_fails {
    () => {
        # [test] # [should_panic (expected = "test did not finish")] fn retry_fails () { retry (2 , | | None :: < () >) ; }
    };
}

retry_fails!()