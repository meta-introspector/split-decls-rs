macro_rules! deps {
    () => {
        Arc!();
        Action!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl From < Action > for rt :: arc :: Action { fn from (action : Action) -> Self { match action { Action :: Arc (action) => action , _ => unreachable ! () , } } }
    };
}

impl_96!()