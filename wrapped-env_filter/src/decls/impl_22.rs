macro_rules! deps {
    () => {
        ParseResult!();
        FilterOp!();
        ParseError!();
        Directive!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl ParseResult { fn add_directive (& mut self , directive : Directive) { self . directives . push (directive) ; } fn set_filter (& mut self , filter : FilterOp) { self . filter = Some (filter) ; } fn add_error (& mut self , message : String) { self . errors . push (message) ; } pub (crate) fn ok (self) -> Result < (Vec < Directive > , Option < FilterOp >) , ParseError > { let Self { directives , filter , errors , } = self ; if let Some (error) = errors . into_iter () . next () { Err (ParseError { details : error }) } else { Ok ((directives , filter)) } } }
    };
}

impl_22!()