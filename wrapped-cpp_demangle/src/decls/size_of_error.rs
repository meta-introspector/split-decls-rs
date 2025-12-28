macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! size_of_error {
    () => {
        deps!();
        # [test] fn size_of_error () { assert_eq ! (core :: mem :: size_of ::< Error > () , 1 , "We should keep the size of our Error type in check") ; }
    };
}

size_of_error!();