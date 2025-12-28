macro_rules! deps {
    () => {
        ArgLineCommandExt!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ArgLineCommandExt for & mut ProcessBuilder { fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self { self . arg (s) } }
    };
}

impl_50!()