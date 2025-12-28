macro_rules! deps {
    () => {
        TargetRef!();
        Target!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > From < TargetRef < 'a > > for Target { fn from (src : TargetRef < 'a >) -> Self { match src { TargetRef :: Object (oid) => Target :: Object (oid . to_owned ()) , TargetRef :: Symbolic (name) => Target :: Symbolic (name . to_owned ()) , } } }
    };
}

impl_70!();