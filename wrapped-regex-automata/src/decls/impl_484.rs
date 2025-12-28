macro_rules! deps {
    () => {
        BuildError!();
        StateID!();
        UnicodeWordBoundaryError!();
        NFA!();
        PatternID!();
        Captures!();
        GroupInfoError!();
        BuildErrorKind!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl BuildError { # [doc = " If this error occurred because the NFA exceeded the configured size"] # [doc = " limit before being built, then this returns the configured size limit."] # [doc = ""] # [doc = " The limit returned is what was configured, and corresponds to the"] # [doc = " maximum amount of heap usage in bytes."] pub fn size_limit (& self) -> Option < usize > { match self . kind { BuildErrorKind :: ExceededSizeLimit { limit } => Some (limit) , _ => None , } } fn kind (& self) -> & BuildErrorKind { & self . kind } # [cfg (feature = "syntax")] pub (crate) fn syntax (err : regex_syntax :: Error) -> BuildError { BuildError { kind : BuildErrorKind :: Syntax (err) } } pub (crate) fn captures (err : captures :: GroupInfoError) -> BuildError { BuildError { kind : BuildErrorKind :: Captures (err) } } pub (crate) fn word (err : look :: UnicodeWordBoundaryError) -> BuildError { BuildError { kind : BuildErrorKind :: Word (err) } } pub (crate) fn too_many_patterns (given : usize) -> BuildError { let limit = PatternID :: LIMIT ; BuildError { kind : BuildErrorKind :: TooManyPatterns { given , limit } } } pub (crate) fn too_many_states (given : usize) -> BuildError { let limit = StateID :: LIMIT ; BuildError { kind : BuildErrorKind :: TooManyStates { given , limit } } } pub (crate) fn exceeded_size_limit (limit : usize) -> BuildError { BuildError { kind : BuildErrorKind :: ExceededSizeLimit { limit } } } pub (crate) fn invalid_capture_index (index : u32) -> BuildError { BuildError { kind : BuildErrorKind :: InvalidCaptureIndex { index } } } # [cfg (feature = "syntax")] pub (crate) fn unsupported_captures () -> BuildError { BuildError { kind : BuildErrorKind :: UnsupportedCaptures } } }
    };
}

impl_484!();