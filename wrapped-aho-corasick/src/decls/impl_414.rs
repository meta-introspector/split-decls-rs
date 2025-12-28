macro_rules! deps {
    () => {
        PatternID!();
        SmallIndex!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl From < PatternID > for SmallIndex { fn from (pid : PatternID) -> SmallIndex { pid . 0 } }
    };
}

impl_414!();