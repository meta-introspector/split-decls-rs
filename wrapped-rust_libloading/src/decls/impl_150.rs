macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl From < imp :: Library > for Library { fn from (lib : imp :: Library) -> Library { Library (lib) } }
    };
}

impl_150!()