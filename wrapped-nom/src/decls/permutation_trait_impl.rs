macro_rules! deps {
    () => {
        Permutation!();
        OutputMode!();
        Input!();
        Error!();
        Mode!();
        Parser!();
        ErrorKind!();
        Err!();
        ParseError!();
        PResult!();
    };
}

macro_rules! permutation_trait_impl {
    () => {
        deps!();
        macro_rules ! permutation_trait_impl (($ ($ name : ident $ ty : ident $ item : ident) ,+) => (impl < Input , Error , $ ($ ty) ,+ , $ ($ name) ,+ > Parser < Input > for Permutation < ($ ($ name) ,+) , Error > where Input : Clone , Error : ParseError < Input >, $ ($ name : Parser < Input , Output = $ ty , Error = Error >) ,+ { type Output = ($ ($ ty) ,+) ; type Error = Error ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , mut input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (|| ($ (Option ::<$ ty >:: None) ,+)) ; $ (let mut $ item = false ;) + loop { let mut err : Option << OM :: Error as Mode >:: Output < Error >> = None ; permutation_trait_inner ! (0 , self , input , res , err , $ ($ item) +) ; if let Some (err) = err { return Err (Err :: Error (OM :: Error :: map (err , | err | Error :: append (input , ErrorKind :: Permutation , err)))) ; } return Ok ((input , OM :: Output :: map (res , | res | { match res { ($ (Some ($ item)) ,+) => ($ ($ item) ,+) , _ => unreachable ! () , } }))) } } }) ;) ;
    };
}

permutation_trait_impl!()