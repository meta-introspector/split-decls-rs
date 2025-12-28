macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! invalid {
    () => {
        deps!();
        # [doc = " Mark the parser as errored (with `ParseError::Invalid`), print the"] # [doc = " appropriate message (see `ParseError::message`) and return early."] macro_rules ! invalid { ($ printer : ident) => { { let err = ParseError :: Invalid ; $ printer . print (err . message ()) ?; $ printer . parser = Err (err) ; return Ok (()) ; } } ; }
    };
}

invalid!()