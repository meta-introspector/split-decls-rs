macro_rules! FoldWithConsumer {
    () => {
        struct FoldWithConsumer < 'c , C , U , F > { base : C , item : U , fold_op : & 'c F , }
    };
}

FoldWithConsumer!();