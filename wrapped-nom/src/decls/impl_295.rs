macro_rules! deps {
    () => {
        Parser!();
        Input!();
        Error!();
        IResult!();
        Tuple!();
        ParseError!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        # [allow (deprecated)] impl < Input , Output , Error : ParseError < Input > , F : Parser < Input , Output = Output , Error = Error > > Tuple < Input , (Output ,) , Error > for (F ,) { fn parse_tuple (& mut self , input : Input) -> IResult < Input , (Output ,) , Error > { self . 0 . parse (input) . map (| (i , o) | (i , (o ,))) } }
    };
}

impl_295!()