macro_rules! deps {
    () => {
        Item!();
        Numeric!();
        Month!();
    };
}

macro_rules! D_FMT {
    () => {
        deps!();
        static D_FMT : & [Item < 'static >] = & [num0 (Numeric :: Month) , Item :: Literal ("/") , num0 (Numeric :: Day) , Item :: Literal ("/") , num0 (Numeric :: YearMod100) ,] ;
    };
}

D_FMT!()