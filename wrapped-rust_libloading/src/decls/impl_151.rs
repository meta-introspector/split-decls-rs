macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl From < Library > for imp :: Library { fn from (lib : Library) -> imp :: Library { lib . 0 } }
    };
}

impl_151!();