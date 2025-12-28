macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Err!();
        Parser!();
    };
}

macro_rules! single_element_tuples {
    () => {
        deps!();
        # [test] fn single_element_tuples () { use crate :: character :: complete :: alpha1 ; use crate :: { error :: ErrorKind , Err } ; let mut parser = (alpha1 ,) ; assert_eq ! (crate :: Parser :: parse (& mut parser , "abc123def") , Ok (("123def" , ("abc" ,)))) ; assert_eq ! (crate :: Parser :: parse (& mut parser , "123def") , Err (Err :: Error (("123def" , ErrorKind :: Alpha)))) ; }
    };
}

single_element_tuples!()