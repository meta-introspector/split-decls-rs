macro_rules! deps {
    () => {
        SmallIndex!();
        StateID!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl From < StateID > for SmallIndex { fn from (sid : StateID) -> SmallIndex { sid . 0 } }
    };
}

impl_413!();