macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! err {
    () => {
        deps!();
        macro_rules ! err { ($ text : expr , $ kind : expr) => { return Err (Error :: new ($ kind , $ text)) } ; ($ text : expr) => { err ! ($ text , ErrorKind :: Other) } ; }
    };
}

err!()