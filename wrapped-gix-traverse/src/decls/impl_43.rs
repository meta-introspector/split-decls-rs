macro_rules! deps {
    () => {
        Location!();
        Recorder!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Default for Recorder { fn default () -> Self { Recorder { path_deque : Default :: default () , path : Default :: default () , location : Location :: Path . into () , records : vec ! [] , } } }
    };
}

impl_43!();