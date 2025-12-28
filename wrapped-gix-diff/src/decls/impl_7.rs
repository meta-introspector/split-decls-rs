macro_rules! deps {
    () => {
        Tracker!();
        Change!();
        Rewrites!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < T : Change > Tracker < T > { # [doc = " Create a new instance with `rewrites` configuration."] pub fn new (rewrites : Rewrites) -> Self { Tracker { items : vec ! [] , path_backing : vec ! [] , rewrites , child_renames : Default :: default () , } } }
    };
}

impl_7!();