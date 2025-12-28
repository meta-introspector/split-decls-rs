macro_rules! endian_reader {
    () => {
        # [cfg (feature = "endian-reader")] mod endian_reader ;
    };
}

endian_reader!()