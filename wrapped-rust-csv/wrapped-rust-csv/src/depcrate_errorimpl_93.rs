// Generated macro for impl_93 (impl)
macro_rules! Depcrate_errorimpl_93 {
() => {
// Module: crate::error
// Provides: {"impl_93"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self . 0 { ErrorKind :: Io (ref err) => err . fmt (f) , ErrorKind :: Utf8 { pos : None , ref err } => { write ! (f , "CSV parse error: field {}: {}" , err . field () , err) } ErrorKind :: Utf8 { pos : Some (ref pos) , ref err } => write ! (f , "CSV parse error: record {} \
                 (line {}, field: {}, byte: {}): {}" , pos . record () , pos . line () , err . field () , pos . byte () , err) , ErrorKind :: UnequalLengths { pos : None , expected_len , len } => { write ! (f , "CSV error: \
                     found record with {} fields, but the previous record \
                     has {} fields" , len , expected_len) } ErrorKind :: UnequalLengths { pos : Some (ref pos) , expected_len , len , } => write ! (f , "CSV error: record {} (line: {}, byte: {}): \
                 found record with {} fields, but the previous record \
                 has {} fields" , pos . record () , pos . line () , pos . byte () , len , expected_len) , ErrorKind :: Seek => write ! (f , "CSV error: cannot access headers of CSV data \
                 when the parser was seeked before the first record \
                 could be read") , ErrorKind :: Serialize (ref err) => { write ! (f , "CSV write error: {}" , err) } ErrorKind :: Deserialize { pos : None , ref err } => { write ! (f , "CSV deserialize error: {}" , err) } ErrorKind :: Deserialize { pos : Some (ref pos) , ref err } => write ! (f , "CSV deserialize error: record {} \
                 (line: {}, byte: {}): {}" , pos . record () , pos . line () , pos . byte () , err) , } } }
};
}
