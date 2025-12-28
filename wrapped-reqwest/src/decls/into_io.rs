macro_rules! deps {
    () => {
        BoxError!();
        Error!();
    };
}

macro_rules! into_io {
    () => {
        deps!();
        # [cfg (any (feature = "gzip" , feature = "zstd" , feature = "brotli" , feature = "deflate" , feature = "blocking" ,))] pub (crate) fn into_io (e : BoxError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) }
    };
}

into_io!();