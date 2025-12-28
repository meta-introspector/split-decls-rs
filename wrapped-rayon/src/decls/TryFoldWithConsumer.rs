macro_rules! TryFoldWithConsumer {
    () => {
        struct TryFoldWithConsumer < 'c , C , U : Try , F > { base : C , item : U :: Output , fold_op : & 'c F , }
    };
}

TryFoldWithConsumer!();