macro_rules! deps {
    () => {
        Value!();
        ValuesMut!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        delegate_iterator ! ((ValuesMut <'a >) => &'a mut Value) ;
    };
}

macro_131!();