macro_rules! deps {
    () => {
        IntoIter!();
        Value!();
    };
}

macro_rules! macro_119 {
    () => {
        deps!();
        delegate_iterator ! ((IntoIter) => (String , Value)) ;
    };
}

macro_119!()