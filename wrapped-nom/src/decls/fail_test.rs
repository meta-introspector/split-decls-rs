macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        Fail!();
        Err!();
    };
}

macro_rules! fail_test {
    () => {
        deps!();
        # [test] fn fail_test () { let a = "string" ; let b = "another string" ; assert_eq ! (fail ::< _ , & str , _ > () . parse (a) , Err (Err :: Error ((a , ErrorKind :: Fail)))) ; assert_eq ! (fail ::< _ , & str , _ > () . parse (b) , Err (Err :: Error ((b , ErrorKind :: Fail)))) ; }
    };
}

fail_test!();