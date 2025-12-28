macro_rules! deps {
    () => {
        DecompressError!();
        Error!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        # [doc = " Implement Error trait only if std feature is requested as it requires std."] # [cfg (all (feature = "std" , feature = "with-alloc"))] impl Error for DecompressError { }
    };
}

impl_207!()