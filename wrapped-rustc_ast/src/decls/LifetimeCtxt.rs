macro_rules! deps {
    () => {
        GenericArg!();
    };
}

macro_rules! LifetimeCtxt {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub enum LifetimeCtxt { # [doc = " Appears in a reference type."] Ref , # [doc = " Appears as a bound on a type or another lifetime."] Bound , # [doc = " Appears as a generic argument."] GenericArg , }
    };
}

LifetimeCtxt!()