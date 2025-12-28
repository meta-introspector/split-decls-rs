macro_rules! deps {
    () => {
        Error!();
        Ignored!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < E : Error > From < E > for Ignored { fn from (_error : E) -> Self { Ignored } }
    };
}

impl_63!()