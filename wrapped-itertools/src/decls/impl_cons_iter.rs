macro_rules! deps {
    () => {
        ConsTuplesFn!();
        MapSpecialCaseFn!();
    };
}

macro_rules! impl_cons_iter {
    () => {
        deps!();
        macro_rules ! impl_cons_iter (($ _A : ident , $ _B : ident ,) => () ; ($ A : ident , $ ($ B : ident ,) *) => (impl_cons_iter ! ($ ($ B ,) *) ; # [allow (non_snake_case)] impl <$ ($ B) ,*, X > MapSpecialCaseFn < (($ ($ B ,) *) , X) > for ConsTuplesFn { type Out = ($ ($ B ,) * X ,) ; fn call (& mut self , (($ ($ B ,) *) , X) : (($ ($ B ,) *) , X)) -> Self :: Out { ($ ($ B ,) * X ,) } }) ;) ;
    };
}

impl_cons_iter!()