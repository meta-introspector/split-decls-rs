macro_rules! BlocksCallback {
    () => {
        struct BlocksCallback < S , C > { sizes : S , consumer : C , len : usize , }
    };
}

BlocksCallback!();