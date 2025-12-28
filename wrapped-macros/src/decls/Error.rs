macro_rules! Error {
    () => {
        enum Error { NonStringLiteral , UuidParse (LitStr , error :: Error) , }
    };
}

Error!();