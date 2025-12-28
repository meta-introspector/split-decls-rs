macro_rules! deps {
    () => {
        PathKind!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl PathKind { pub const SELF : PathKind = PathKind :: Super (0) ; }
    };
}

impl_125!()