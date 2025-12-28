macro_rules! deps {
    () => {
        Metadata!();
        Source!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < Source > for Metadata { fn from (source : Source) -> Self { file :: Metadata { path : None , source , level : 0 , trust : gix_sec :: Trust :: Full , } } }
    };
}

impl_71!();