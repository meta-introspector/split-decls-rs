macro_rules! deps {
    () => {
        Parser!();
        ParseError!();
        Input!();
        Choice!();
        PResult!();
        Err!();
        ErrorKind!();
        OutputMode!();
        Error!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < Input : Clone , Output , Error : ParseError < Input > , A : Parser < Input , Output = Output , Error = Error > , > Parser < Input > for Choice < & mut [A] > { type Output = Output ; type Error = Error ; # [inline] fn process < OM : crate :: OutputMode > (& mut self , input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { let mut error = None ; for branch in self . parser . iter_mut () { match branch . process :: < OM > (input . clone ()) { Err (Err :: Error (e)) => match error { None => error = Some (e) , Some (err) => error = Some (OM :: Error :: combine (err , e , | e1 , e2 | e1 . or (e2))) , } , res => return res , } } match error { Some (e) => Err (Err :: Error (OM :: Error :: map (e , | err | { Error :: append (input , ErrorKind :: Alt , err) }))) , None => Err (Err :: Error (OM :: Error :: bind (| | { Error :: from_error_kind (input , ErrorKind :: Alt) }))) , } } }
    };
}

impl_60!();