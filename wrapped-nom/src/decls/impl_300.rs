macro_rules! deps {
    () => {
        Tuple!();
        IResult!();
        ParseError!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        # [allow (deprecated)] impl < I , E : ParseError < I > > Tuple < I , () , E > for () { fn parse_tuple (& mut self , input : I) -> IResult < I , () , E > { Ok ((input , ())) } }
    };
}

impl_300!()