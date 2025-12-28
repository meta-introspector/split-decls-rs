macro_rules! deps {
    () => {
        Numeric!();
        Item!();
    };
}

macro_rules! T_FMT {
    () => {
        deps!();
        static T_FMT : & [Item < 'static >] = & [num0 (Numeric :: Hour) , Item :: Literal (":") , num0 (Numeric :: Minute) , Item :: Literal (":") , num0 (Numeric :: Second) ,] ;
    };
}

T_FMT!()