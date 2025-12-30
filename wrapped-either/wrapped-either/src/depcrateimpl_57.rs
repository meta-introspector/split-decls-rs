// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
# [doc = " Convert from `Result` to `Either` with `Ok => Right` and `Err => Left`."] impl < L , R > From < Result < R , L > > for Either < L , R > { fn from (r : Result < R , L >) -> Self { match r { Err (e) => Left (e) , Ok (o) => Right (o) , } } }
};
}
