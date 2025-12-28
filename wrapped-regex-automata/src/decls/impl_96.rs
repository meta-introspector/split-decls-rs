macro_rules! deps {
    () => {
        NFA!();
        Look!();
        BuildError!();
        BuildErrorKind!();
        UnicodeWordBoundaryError!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl BuildError { fn nfa (err : crate :: nfa :: thompson :: BuildError) -> BuildError { BuildError { kind : BuildErrorKind :: NFA (err) } } fn word (err : UnicodeWordBoundaryError) -> BuildError { BuildError { kind : BuildErrorKind :: Word (err) } } fn too_many_states (limit : u64) -> BuildError { BuildError { kind : BuildErrorKind :: TooManyStates { limit } } } fn too_many_patterns (limit : u64) -> BuildError { BuildError { kind : BuildErrorKind :: TooManyPatterns { limit } } } fn unsupported_look (look : Look) -> BuildError { BuildError { kind : BuildErrorKind :: UnsupportedLook { look } } } fn exceeded_size_limit (limit : usize) -> BuildError { BuildError { kind : BuildErrorKind :: ExceededSizeLimit { limit } } } fn not_one_pass (msg : & 'static str) -> BuildError { BuildError { kind : BuildErrorKind :: NotOnePass { msg } } } }
    };
}

impl_96!()