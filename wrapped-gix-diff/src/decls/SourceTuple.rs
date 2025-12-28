macro_rules! deps {
    () => {
        Item!();
        DiffLineStats!();
    };
}

macro_rules! SourceTuple {
    () => {
        deps!();
        # [doc = " <`src_idx`, src, possibly diff stat>"] type SourceTuple < 'a , T > = (usize , & 'a Item < T > , Option < DiffLineStats >) ;
    };
}

SourceTuple!()