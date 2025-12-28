macro_rules! deps {
    () => {
        Permissions!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        impl gix_sec :: trust :: DefaultForLevel for Permissions { fn default_for_level (level : Trust) -> Self { match level { Trust :: Full => Permissions :: all () , Trust :: Reduced => Permissions :: secure () , } } }
    };
}

impl_513!();