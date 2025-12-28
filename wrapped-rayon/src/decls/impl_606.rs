macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        # [doc = " Collects OS-string slices from a parallel iterator into an OS-string."] impl < 'a > FromParallelIterator < & 'a OsStr > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = & 'a OsStr > , { collect_extended (par_iter) } }
    };
}

impl_606!();