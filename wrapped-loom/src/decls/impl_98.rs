macro_rules! deps {
    () => {
        Channel!();
        Action!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl From < Action > for rt :: mpsc :: Action { fn from (action : Action) -> Self { match action { Action :: Channel (action) => action , _ => unreachable ! () , } } }
    };
}

impl_98!()