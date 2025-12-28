macro_rules! ncb {
    () => {
        fn ncb (shard_amount : usize) -> usize { shard_amount . trailing_zeros () as usize }
    };
}

ncb!()