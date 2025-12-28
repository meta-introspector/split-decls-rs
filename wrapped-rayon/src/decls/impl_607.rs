macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        # [doc = " Collects OS-strings from a parallel iterator into one large OS-string."] impl FromParallelIterator < OsString > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = OsString > , { collect_extended (par_iter) } }
    };
}

impl_607!()