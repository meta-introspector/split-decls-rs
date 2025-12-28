macro_rules! private {
    () => {
        mod private { pub trait Sealed { } impl Sealed for std :: ffi :: OsStr { } }
    };
}

private!()