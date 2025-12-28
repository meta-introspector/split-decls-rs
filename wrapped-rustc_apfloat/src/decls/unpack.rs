macro_rules! deps {
    () => {
        StatusAnd!();
    };
}

macro_rules! unpack {
    () => {
        deps!();
        # [macro_export] macro_rules ! unpack { ($ status : ident |=, $ e : expr) => { match $ e { $ crate :: StatusAnd { status , value } => { $ status |= status ; value } } } ; ($ status : ident =, $ e : expr) => { match $ e { $ crate :: StatusAnd { status , value } => { $ status = status ; value } } } ; }
    };
}

unpack!();