macro_rules! deps {
    () => {
        Reversed!();
    };
}

macro_rules! macro_192 {
    () => {
        deps!();
        EdgeCount ! { delegate_impl [[G] , G , Reversed < G >, access0] }
    };
}

macro_192!()