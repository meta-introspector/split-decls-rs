macro_rules! FoldConsumer {
    () => {
        struct FoldConsumer < 'c , C , ID , F > { base : C , fold_op : & 'c F , identity : & 'c ID , }
    };
}

FoldConsumer!();