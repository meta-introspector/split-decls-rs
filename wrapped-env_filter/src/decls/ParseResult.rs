macro_rules! deps {
    () => {
        FilterOp!();
        Directive!();
    };
}

macro_rules! ParseResult {
    () => {
        deps!();
        # [derive (Default , Debug)] pub (crate) struct ParseResult { pub (crate) directives : Vec < Directive > , pub (crate) filter : Option < FilterOp > , pub (crate) errors : Vec < String > , }
    };
}

ParseResult!()