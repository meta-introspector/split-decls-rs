macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: AccessError { path , inner } => { write ! (f , "Failed to access file {:?}: {}" , path , inner) } Error :: CopyError { from , to , inner } => { write ! (f , "Failed to copy file {:?} to {:?}: {}" , from , to , inner) } Error :: SerdeError { path , inner } => write ! (f , "Failed to read or write file {:?} due to serialization error: {}" , path , inner) , # [cfg (feature = "csv_output")] Error :: CsvError (inner) => write ! (f , "CSV error: {}" , inner) , } } }
    };
}

impl_87!();