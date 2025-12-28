macro_rules! deps {
    () => {
        Err!();
        Error!();
    };
}

macro_rules! IResult {
    () => {
        deps!();
        # [doc = " Holds the result of parsing functions"] # [doc = ""] # [doc = " It depends on the input type `I`, the output type `O`, and the error type `E`"] # [doc = " (by default `(I, nom::ErrorKind)`)"] # [doc = ""] # [doc = " The `Ok` side is a pair containing the remainder of the input (the part of the data that"] # [doc = " was not parsed) and the produced value. The `Err` side contains an instance of `nom::Err`."] # [doc = ""] # [doc = " Outside of the parsing code, you can use the [Finish::finish] method to convert"] # [doc = " it to a more common result type"] pub type IResult < I , O , E = error :: Error < I > > = Result < (I , O) , Err < E > > ;
    };
}

IResult!();