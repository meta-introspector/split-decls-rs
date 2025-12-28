macro_rules! deps {
    () => {
        Fixed!();
        Item!();
        Numeric!();
    };
}

macro_rules! T_FMT_AMPM {
    () => {
        deps!();
        static T_FMT_AMPM : & [Item < 'static >] = & [num0 (Numeric :: Hour12) , Item :: Literal (":") , num0 (Numeric :: Minute) , Item :: Literal (":") , num0 (Numeric :: Second) , Item :: Space (" ") , fixed (Fixed :: UpperAmPm) ,] ;
    };
}

T_FMT_AMPM!()