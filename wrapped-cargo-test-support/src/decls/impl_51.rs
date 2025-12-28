macro_rules! deps {
    () => {
        Execs!();
        ArgLineCommandExt!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ArgLineCommandExt for & mut Execs { fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self { self . arg (s) } }
    };
}

impl_51!()