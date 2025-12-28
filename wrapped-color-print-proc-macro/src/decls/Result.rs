macro_rules! deps {
    () => {
        Input!();
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        pub type Result < 'a , V > = IResult < Input < 'a > , V , Error < 'a > > ;
    };
}

Result!()