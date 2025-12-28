macro_rules! TryFoldConsumer {
    () => {
        struct TryFoldConsumer < 'c , U , C , ID , F > { base : C , identity : & 'c ID , fold_op : & 'c F , marker : PhantomData < U > , }
    };
}

TryFoldConsumer!();