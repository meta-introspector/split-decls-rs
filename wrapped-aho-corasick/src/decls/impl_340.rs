macro_rules! deps {
    () => {
        ErrorKind!();
        PatternID!();
        BuildError!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl BuildError { pub (crate) fn state_id_overflow (max : u64 , requested_max : u64 ,) -> BuildError { BuildError { kind : ErrorKind :: StateIDOverflow { max , requested_max } } } pub (crate) fn pattern_id_overflow (max : u64 , requested_max : u64 ,) -> BuildError { BuildError { kind : ErrorKind :: PatternIDOverflow { max , requested_max } , } } pub (crate) fn pattern_too_long (pattern : PatternID , len : usize ,) -> BuildError { BuildError { kind : ErrorKind :: PatternTooLong { pattern , len } } } }
    };
}

impl_340!()