macro_rules! deps {
    () => {
        ResponseError!();
        RequestId!();
        Response!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Response { pub fn new_ok < R : serde :: Serialize > (id : RequestId , result : R) -> Response { Response { id , result : Some (serde_json :: to_value (result) . unwrap ()) , error : None } } pub fn new_err (id : RequestId , code : i32 , message : String) -> Response { let error = ResponseError { code , message , data : None } ; Response { id , result : None , error : Some (error) } } }
    };
}

impl_27!();