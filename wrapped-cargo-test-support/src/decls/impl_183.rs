macro_rules! deps {
    () => {
        ArgLineCommandExt!();
        Execs!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl ArgLineCommandExt for & mut Execs { fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self { self . arg (s) } }
    };
}

impl_183!();