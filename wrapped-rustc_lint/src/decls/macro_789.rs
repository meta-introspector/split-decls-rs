macro_rules! deps {
    () => {
        CheckTransmutes!();
    };
}

macro_rules! macro_789 {
    () => {
        deps!();
        impl_lint_pass ! (CheckTransmutes => [PTR_TO_INTEGER_TRANSMUTE_IN_CONSTS , UNNECESSARY_TRANSMUTES , INTEGER_TO_PTR_TRANSMUTES]) ;
    };
}

macro_789!();