macro_rules! deps {
    () => {
        Input!();
        Choice!();
        Err!();
        Parser!();
        OutputMode!();
        Error!();
        ParseError!();
        PResult!();
    };
}

macro_rules! alt_trait_impl {
    () => {
        deps!();
        macro_rules ! alt_trait_impl (($ ($ id : ident) +) => (impl < Input : Clone , Output , Error : ParseError < Input >, $ ($ id : Parser < Input , Output = Output , Error = Error >) ,+ > Parser < Input > for Choice < ($ ($ id) ,+) > { type Output = Output ; type Error = Error ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { match self . parser . 0 . process ::< OM > (input . clone ()) { Ok (res) => Ok (res) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Err (Err :: Error (e)) => alt_trait_inner ! (1 , self , input , e , $ ($ id) +) , } } }) ;) ;
    };
}

alt_trait_impl!();