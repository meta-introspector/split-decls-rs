// Generated macro for impl_156 (impl)
macro_rules! Depcrate_errorimpl_156 {
() => {
// Module: crate::error
// Provides: {"impl_156"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: AccessError { path , inner } => { write ! (f , "Failed to access file {:?}: {}" , path , inner) } Error :: CopyError { from , to , inner } => { write ! (f , "Failed to copy file {:?} to {:?}: {}" , from , to , inner) } Error :: SerdeError { path , inner } => write ! (f , "Failed to read or write file {:?} due to serialization error: {}" , path , inner) , # [cfg (feature = "csv_output")] Error :: CsvError (inner) => write ! (f , "CSV error: {}" , inner) , } } }
};
}
