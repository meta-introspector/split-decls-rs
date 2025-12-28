macro_rules! deps {
    () => {
        Parser!();
        IResult!();
        ParseError!();
        Input!();
        Error!();
        Tuple!();
    };
}

macro_rules! tuple_trait_impl {
    () => {
        deps!();
        macro_rules ! tuple_trait_impl (($ ($ name : ident $ ty : ident) ,+) => (# [allow (deprecated)] impl < Input : Clone , $ ($ ty) ,+ , Error : ParseError < Input >, $ ($ name : Parser < Input , Output = $ ty , Error = Error >) ,+ > Tuple < Input , ($ ($ ty) ,+) , Error > for ($ ($ name) ,+) { fn parse_tuple (& mut self , input : Input) -> IResult < Input , ($ ($ ty) ,+) , Error > { tuple_trait_inner ! (0 , self , input , () , $ ($ name) +) } }) ;) ;
    };
}

tuple_trait_impl!();