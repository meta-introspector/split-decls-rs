macro_rules! deps {
    () => {
        AutoderefKind!();
    };
}

macro_rules! TrackAutoderefSteps {
    () => {
        deps!();
        pub (crate) trait TrackAutoderefSteps < 'db > : Default + fmt :: Debug { fn len (& self) -> usize ; fn push (& mut self , ty : Ty < 'db > , kind : AutoderefKind) ; }
    };
}

TrackAutoderefSteps!();