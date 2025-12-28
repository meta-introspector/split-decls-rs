macro_rules! deps {
    () => {
        Op!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl Op { fn span (& self) -> Span { match self { Op :: BinOp (op) => op . span , Op :: AssignOp (op) => op . span , } } fn as_str (& self) -> & 'static str { match self { Op :: BinOp (op) => op . node . as_str () , Op :: AssignOp (op) => op . node . as_str () , } } fn is_by_value (& self) -> bool { match self { Op :: BinOp (op) => op . node . is_by_value () , Op :: AssignOp (op) => op . node . is_by_value () , } } }
    };
}

impl_304!();