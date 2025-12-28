macro_rules! tuples_try_from_row {
    () => {
        macro_rules ! tuples_try_from_row { () => { tuple_try_from_row ! () ; } ; ($ first : ident $ (, $ remaining : ident) *) => { tuple_try_from_row ! ($ first $ (, $ remaining) *) ; tuples_try_from_row ! ($ ($ remaining) ,*) ; } ; }
    };
}

tuples_try_from_row!()