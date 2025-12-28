macro_rules! deps {
    () => {
        Recorder!();
        Location!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Default for Recorder { fn default () -> Self { Recorder { path_deque : Default :: default () , path : Default :: default () , location : Some (Location :: Path) , records : vec ! [] , } } }
    };
}

impl_53!()