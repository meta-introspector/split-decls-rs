macro_rules! deps {
    () => {
        FromSqlError!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl PartialEq for FromSqlError { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: InvalidType , Self :: InvalidType) => true , (Self :: OutOfRange (n1) , Self :: OutOfRange (n2)) => n1 == n2 , (Self :: InvalidBlobSize { expected_size : es1 , blob_size : bs1 , } , Self :: InvalidBlobSize { expected_size : es2 , blob_size : bs2 , } ,) => es1 == es2 && bs1 == bs2 , (..) => false , } } }
    };
}

impl_323!();