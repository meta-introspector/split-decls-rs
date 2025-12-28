macro_rules! deps {
    () => {
        Action!();
        Atomic!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl From < Action > for rt :: atomic :: Action { fn from (action : Action) -> Self { match action { Action :: Atomic (action) => action , _ => unreachable ! () , } } }
    };
}

impl_97!();