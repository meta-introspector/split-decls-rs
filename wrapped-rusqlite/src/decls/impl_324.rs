macro_rules! deps {
    () => {
        Result!();
        FromSqlError!();
        Value!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl fmt :: Display for FromSqlError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: InvalidType => write ! (f , "Invalid type") , Self :: OutOfRange (i) => write ! (f , "Value {i} out of range") , Self :: InvalidBlobSize { expected_size , blob_size , } => { write ! (f , "Cannot read {expected_size} byte value out of {blob_size} byte blob") } Self :: Other (ref err) => err . fmt (f) , } } }
    };
}

impl_324!();