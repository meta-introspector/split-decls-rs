// Generated macro for impl_157 (impl)
macro_rules! Depcrate_errorimpl_157 {
() => {
// Module: crate::error
// Provides: {"impl_157"}
// Dependencies: {}
impl StdError for Error { fn description (& self) -> & str { match self { Error :: AccessError { .. } => "AccessError" , Error :: CopyError { .. } => "CopyError" , Error :: SerdeError { .. } => "SerdeError" , # [cfg (feature = "csv_output")] Error :: CsvError (_) => "CsvError" , } } fn cause (& self) -> Option < & dyn StdError > { match self { Error :: AccessError { inner , .. } => Some (inner) , Error :: CopyError { inner , .. } => Some (inner) , Error :: SerdeError { inner , .. } => Some (inner) , # [cfg (feature = "csv_output")] Error :: CsvError (inner) => Some (inner) , } } }
};
}
