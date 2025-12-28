macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        # [doc = " Collects OS-string slices from a parallel iterator into an OS-string."] impl < 'a > FromParallelIterator < Cow < 'a , OsStr > > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = Cow < 'a , OsStr > > , { collect_extended (par_iter) } }
    };
}

impl_608!()