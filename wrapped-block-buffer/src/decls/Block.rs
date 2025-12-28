macro_rules! Block {
    () => {
        type Block < N > = MaybeUninit < Array < u8 , N > > ;
    };
}

Block!()