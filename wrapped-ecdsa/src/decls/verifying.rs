macro_rules! verifying {
    () => {
        # [cfg (feature = "algorithm")] mod verifying ;
    };
}

verifying!();