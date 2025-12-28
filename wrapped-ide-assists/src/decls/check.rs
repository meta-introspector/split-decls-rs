macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check {
    () => {
        deps!();
        # [track_caller] fn check (handler : Handler , before : & str , expected : ExpectedResult < '_ > , assist_label : Option < & str >) { check_with_config (TEST_CONFIG , handler , before , expected , assist_label) ; }
    };
}

check!()