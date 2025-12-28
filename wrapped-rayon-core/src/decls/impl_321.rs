macro_rules! deps {
    () => {
        ErrorKind!();
        ThreadPoolBuildError!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl ThreadPoolBuildError { fn new (kind : ErrorKind) -> ThreadPoolBuildError { ThreadPoolBuildError { kind } } fn is_unsupported (& self) -> bool { matches ! (& self . kind , ErrorKind :: IOError (e) if e . kind () == io :: ErrorKind :: Unsupported) } }
    };
}

impl_321!();