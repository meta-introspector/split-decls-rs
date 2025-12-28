macro_rules! writer_mt {
    () => {
        # [cfg (all (feature = "encoder" , feature = "std"))] mod writer_mt ;
    };
}

writer_mt!()