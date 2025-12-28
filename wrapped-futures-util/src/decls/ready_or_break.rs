macro_rules! deps {
    () => {
        Pending!();
        Ready!();
    };
}

macro_rules! ready_or_break {
    () => {
        deps!();
        macro_rules ! ready_or_break { ($ e : expr $ (,) ?) => { match $ e { $ crate :: task :: Poll :: Ready (t) => t , $ crate :: task :: Poll :: Pending => break , } } ; }
    };
}

ready_or_break!();