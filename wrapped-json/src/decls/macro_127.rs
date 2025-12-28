macro_rules! deps {
    () => {
        Values!();
        Value!();
    };
}

macro_rules! macro_127 {
    () => {
        deps!();
        delegate_iterator ! ((Values <'a >) => &'a Value) ;
    };
}

macro_127!();