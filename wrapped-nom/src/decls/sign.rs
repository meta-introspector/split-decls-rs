macro_rules! deps {
    () => {
        Compare!();
        Input!();
        ParseError!();
        IResult!();
    };
}

macro_rules! sign {
    () => {
        deps!();
        pub (crate) fn sign < T , E : ParseError < T > > (input : T) -> IResult < T , bool , E > where T : Clone + Input , T : for < 'a > Compare < & 'a [u8] > , { use crate :: bytes :: streaming :: tag ; use crate :: combinator :: value ; let (i , opt_sign) = opt (alt ((value (false , tag (& b"-" [..])) , value (true , tag (& b"+" [..])) ,))) . parse (input) ? ; let sign = opt_sign . unwrap_or (true) ; Ok ((i , sign)) }
    };
}

sign!()