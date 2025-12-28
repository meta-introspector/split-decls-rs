macro_rules! deps {
    () => {
        Response!();
        RequestId!();
        Incoming!();
        ResponseError!();
        ErrorCode!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < I > Incoming < I > { pub fn register (& mut self , id : RequestId , data : I) { self . pending . insert (id , data) ; } pub fn cancel (& mut self , id : RequestId) -> Option < Response > { let _data = self . complete (& id) ? ; let error = ResponseError { code : ErrorCode :: RequestCanceled as i32 , message : "canceled by client" . to_owned () , data : None , } ; Some (Response { id , result : None , error : Some (error) }) } pub fn complete (& mut self , id : & RequestId) -> Option < I > { self . pending . remove (id) } pub fn is_completed (& self , id : & RequestId) -> bool { ! self . pending . contains_key (id) } }
    };
}

impl_38!()