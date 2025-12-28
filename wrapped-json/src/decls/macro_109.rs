macro_rules! deps {
    () => {
        Value!();
        Iter!();
    };
}

macro_rules! macro_109 {
    () => {
        deps!();
        delegate_iterator ! ((Iter <'a >) => (&'a String , &'a Value)) ;
    };
}

macro_109!();