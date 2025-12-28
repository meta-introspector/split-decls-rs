macro_rules! deps {
    () => {
        IntoValues!();
        Value!();
    };
}

macro_rules! macro_135 {
    () => {
        deps!();
        delegate_iterator ! ((IntoValues) => Value) ;
    };
}

macro_135!();