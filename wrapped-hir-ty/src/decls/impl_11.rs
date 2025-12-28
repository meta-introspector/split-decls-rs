macro_rules! deps {
    () => {
        AutoderefKind!();
        TrackAutoderefSteps!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'db > TrackAutoderefSteps < 'db > for Vec < (Ty < 'db > , AutoderefKind) > { fn len (& self) -> usize { self . len () } fn push (& mut self , ty : Ty < 'db > , kind : AutoderefKind) { self . push ((ty , kind)) ; } }
    };
}

impl_11!()