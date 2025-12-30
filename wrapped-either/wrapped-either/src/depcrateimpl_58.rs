// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
# [doc = " Convert from `Either` to `Result` with `Right => Ok` and `Left => Err`."] impl < L , R > From < Either < L , R > > for Result < R , L > { fn from (val : Either < L , R >) -> Self { match val { Left (l) => Err (l) , Right (r) => Ok (r) , } } }
};
}
