macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Default for Conflict { fn default () -> Self { Conflict :: Keep { style : Default :: default () , marker_size : Conflict :: DEFAULT_MARKER_SIZE . try_into () . unwrap () , } } }
    };
}

impl_11!()