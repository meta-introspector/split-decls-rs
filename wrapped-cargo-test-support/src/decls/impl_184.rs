macro_rules! deps {
    () => {
        ArgLineCommandExt!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl ArgLineCommandExt for snapbox :: cmd :: Command { fn arg < S : AsRef < std :: ffi :: OsStr > > (self , s : S) -> Self { self . arg (s) } }
    };
}

impl_184!();