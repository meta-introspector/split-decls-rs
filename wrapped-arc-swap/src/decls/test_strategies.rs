macro_rules! test_strategies {
    () => {
        # [cfg (feature = "internal-test-strategies")] # [doc (hidden)] pub mod test_strategies ;
    };
}

test_strategies!();