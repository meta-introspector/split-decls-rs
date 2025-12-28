macro_rules! deps {
    () => {
        LocalTimeType!();
        Fixed!();
        TransitionRule!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl From < LocalTimeType > for TransitionRule { fn from (inner : LocalTimeType) -> Self { TransitionRule :: Fixed (inner) } }
    };
}

impl_610!()