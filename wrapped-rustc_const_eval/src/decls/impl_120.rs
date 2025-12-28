macro_rules! deps {
    () => {
        CanAccessMutGlobal!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl From < bool > for CanAccessMutGlobal { fn from (value : bool) -> Self { if value { Self :: Yes } else { Self :: No } } }
    };
}

impl_120!()