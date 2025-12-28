macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Out {
    () => {
        deps!();
        pub struct Out (Any) ;
    };
}

Out!();