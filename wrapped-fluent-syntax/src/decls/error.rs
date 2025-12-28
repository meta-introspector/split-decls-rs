macro_rules! deps {
    () => {
        ParserError!();
    };
}

macro_rules! error {
    () => {
        deps!();
        macro_rules ! error { ($ kind : expr , $ start : expr) => { { Err (ParserError { pos : $ start ..$ start + 1 , slice : None , kind : $ kind , }) } } ; ($ kind : expr , $ start : expr , $ end : expr) => { { Err (ParserError { pos : $ start ..$ end , slice : None , kind : $ kind , }) } } ; }
    };
}

error!()