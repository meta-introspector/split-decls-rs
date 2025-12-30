// Generated macro for impl_592 (impl)
macro_rules! Depcrateimpl_592 {
() => {
// Module: crate
// Provides: {"impl_592"}
// Dependencies: {}
impl Error { fn new (message : String) -> Error { Error { message } } pub fn into_string (self) -> String { self . into () } # [doc = " Parse corresponding [`ErrorKind`] from error message."] pub fn kind (& self) -> ErrorKind { match self . message . split (':') . next () . unwrap_or ("") { "NotFound" => ErrorKind :: NotFound , "Corruption" => ErrorKind :: Corruption , "Not implemented" => ErrorKind :: NotSupported , "Invalid argument" => ErrorKind :: InvalidArgument , "IO error" => ErrorKind :: IOError , "Merge in progress" => ErrorKind :: MergeInProgress , "Result incomplete" => ErrorKind :: Incomplete , "Shutdown in progress" => ErrorKind :: ShutdownInProgress , "Operation timed out" => ErrorKind :: TimedOut , "Operation aborted" => ErrorKind :: Aborted , "Resource busy" => ErrorKind :: Busy , "Operation expired" => ErrorKind :: Expired , "Operation failed. Try again." => ErrorKind :: TryAgain , "Compaction too large" => ErrorKind :: CompactionTooLarge , "Column family dropped" => ErrorKind :: ColumnFamilyDropped , _ => ErrorKind :: Unknown , } } }
};
}
