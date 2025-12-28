macro_rules! deps {
    () => {
        Input!();
        Result!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        pub trait Parser < 'a , V > : FnMut (Input < 'a >) -> Result < 'a , V > { }
    };
}

Parser!();