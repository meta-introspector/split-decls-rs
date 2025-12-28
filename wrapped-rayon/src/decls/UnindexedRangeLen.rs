macro_rules! UnindexedRangeLen {
    () => {
        trait UnindexedRangeLen < L > { fn unindexed_len (& self) -> L ; }
    };
}

UnindexedRangeLen!()