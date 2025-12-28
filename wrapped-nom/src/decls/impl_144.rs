macro_rules! deps {
    () => {
        Error!();
        Finish!();
        Err!();
        IResult!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < I , O , E > Finish < I , O , E > for IResult < I , O , E > { fn finish (self) -> Result < (I , O) , E > { match self { Ok (res) => Ok (res) , Err (Err :: Error (e)) | Err (Err :: Failure (e)) => Err (e) , Err (Err :: Incomplete (_)) => { panic ! ("Cannot call `finish()` on `Err(Err::Incomplete(_))`: this result means that the parser does not have enough data to decide, you should gather more data and try to reapply the parser instead") } } } }
    };
}

impl_144!();