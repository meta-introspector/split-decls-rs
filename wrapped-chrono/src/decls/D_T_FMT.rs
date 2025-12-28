macro_rules! deps {
    () => {
        Item!();
        Fixed!();
        Numeric!();
    };
}

macro_rules! D_T_FMT {
    () => {
        deps!();
        static D_T_FMT : & [Item < 'static >] = & [fixed (Fixed :: ShortWeekdayName) , Item :: Space (" ") , fixed (Fixed :: ShortMonthName) , Item :: Space (" ") , nums (Numeric :: Day) , Item :: Space (" ") , num0 (Numeric :: Hour) , Item :: Literal (":") , num0 (Numeric :: Minute) , Item :: Literal (":") , num0 (Numeric :: Second) , Item :: Space (" ") , num0 (Numeric :: Year) ,] ;
    };
}

D_T_FMT!()