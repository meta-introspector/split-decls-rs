macro_rules! deps {
    () => {
        ErrorKind!();
        IResult!();
    };
}

macro_rules! assert_parse {
    () => {
        deps!();
        macro_rules ! assert_parse (($ left : expr , $ right : expr) => { let res : $ crate :: IResult < _ , _ , (_ , ErrorKind) > = $ left ; assert_eq ! (res , $ right) ; } ;) ;
    };
}

assert_parse!();