// Generated macro for impl_1001 (impl)
macro_rules! Depcrateimpl_1001 {
() => {
// Module: crate
// Provides: {"impl_1001"}
// Dependencies: {}
impl < T > OptionalExtension < T > for Result < T > { fn optional (self) -> Result < Option < T > > { match self { Ok (value) => Ok (Some (value)) , Err (Error :: QueryReturnedNoRows) => Ok (None) , Err (e) => Err (e) , } } }
};
}
