macro_rules! deps {
    () => {
        AlternateTime!();
        TransitionRule!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl From < AlternateTime > for TransitionRule { fn from (inner : AlternateTime) -> Self { TransitionRule :: Alternate (inner) } }
    };
}

impl_611!();