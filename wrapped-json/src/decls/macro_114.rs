macro_rules! deps {
    () => {
        IterMut!();
        Value!();
    };
}

macro_rules! macro_114 {
    () => {
        deps!();
        delegate_iterator ! ((IterMut <'a >) => (&'a String , &'a mut Value)) ;
    };
}

macro_114!()