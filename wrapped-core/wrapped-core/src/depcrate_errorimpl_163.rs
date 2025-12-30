// Generated macro for impl_163 (impl)
macro_rules! Depcrate_errorimpl_163 {
() => {
// Module: crate::error
// Provides: {"impl_163"}
// Dependencies: {}
impl < T > ResultDataError < T > for Result < T , DataError > { fn allow_identifier_not_found (self) -> Result < Option < T > , DataError > { match self { Ok (t) => Ok (Some (t)) , Err (DataError { kind : DataErrorKind :: IdentifierNotFound , .. }) => Ok (None) , Err (e) => Err (e) , } } }
};
}
