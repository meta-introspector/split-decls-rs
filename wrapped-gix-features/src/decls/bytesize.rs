macro_rules! bytesize {
    () => {
        # [doc = " A stub for the portions of the `bytesize` crate that we use internally in `gitoxide`."] # [cfg (not (feature = "progress-unit-bytes"))] pub mod bytesize { # [doc = " A stub for the `ByteSize` wrapper."] pub struct ByteSize (pub u64) ; impl std :: fmt :: Display for ByteSize { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } } }
    };
}

bytesize!();