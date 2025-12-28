macro_rules! deps {
    () => {
        Duration!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl From < std :: time :: Duration > for Duration { fn from (other : std :: time :: Duration) -> Self { Duration { secs : other . as_secs () , nanos : other . subsec_nanos () , } } }
    };
}

impl_73!();