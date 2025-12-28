macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! macro_123 {
    () => {
        deps!();
        delegate_iterator ! ((Keys <'a >) => &'a String) ;
    };
}

macro_123!();