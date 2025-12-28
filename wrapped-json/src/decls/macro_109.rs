macro_rules! deps {
    () => {
        Iter!();
        Value!();
    };
}

macro_rules! macro_109 {
    () => {
        deps!();
        delegate_iterator ! ((Iter <'a >) => (&'a String , &'a Value)) ;
    };
}

macro_109!()