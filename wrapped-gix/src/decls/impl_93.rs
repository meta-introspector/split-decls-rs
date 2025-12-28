macro_rules! deps {
    () => {
        Repository!();
        PrepareFetch!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl From < PrepareFetch > for Repository { fn from (prep : PrepareFetch) -> Self { prep . persist () } }
    };
}

impl_93!()