// Generated macro for impl_381 (impl)
macro_rules! Depcrate_util_errorimpl_381 {
() => {
// Module: crate::util::error
// Provides: {"impl_381"}
// Dependencies: {}
impl BuildError { pub (crate) fn state_id_overflow (max : u64 , requested_max : u64 ,) -> BuildError { BuildError { kind : ErrorKind :: StateIDOverflow { max , requested_max } } } pub (crate) fn pattern_id_overflow (max : u64 , requested_max : u64 ,) -> BuildError { BuildError { kind : ErrorKind :: PatternIDOverflow { max , requested_max } , } } pub (crate) fn pattern_too_long (pattern : PatternID , len : usize ,) -> BuildError { BuildError { kind : ErrorKind :: PatternTooLong { pattern , len } } } }
};
}
