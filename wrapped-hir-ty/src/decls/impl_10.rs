macro_rules! deps {
    () => {
        TrackAutoderefSteps!();
        AutoderefKind!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'db > TrackAutoderefSteps < 'db > for usize { fn len (& self) -> usize { * self } fn push (& mut self , _ : Ty < 'db > , _ : AutoderefKind) { * self += 1 ; } }
    };
}

impl_10!();