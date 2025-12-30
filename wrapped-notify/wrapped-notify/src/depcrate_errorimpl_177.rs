// Generated macro for impl_177 (impl)
macro_rules! Depcrate_errorimpl_177 {
() => {
// Module: crate::error
// Provides: {"impl_177"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let error = match self . kind { ErrorKind :: PathNotFound => "No path was found." . into () , ErrorKind :: WatchNotFound => "No watch was found." . into () , ErrorKind :: InvalidConfig (ref config) => format ! ("Invalid configuration: {:?}" , config) , ErrorKind :: Generic (ref err) => err . clone () , ErrorKind :: Io (ref err) => err . to_string () , ErrorKind :: MaxFilesWatch => "OS file watch limit reached." . into () , } ; if self . paths . is_empty () { write ! (f , "{}" , error) } else { write ! (f , "{} about {:?}" , error , self . paths) } } }
};
}
