macro_rules! deps {
    () => {
        ParseMonthError!();
        Month!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        # [doc = " Parsing a `str` into a `Month` uses the format [`%B`](./format/strftime/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::Month;"] # [doc = ""] # [doc = " assert_eq!(\"January\".parse::<Month>(), Ok(Month::January));"] # [doc = " assert!(\"any day\".parse::<Month>().is_err());"] # [doc = " ```"] # [doc = ""] # [doc = " The parsing is case-insensitive."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::Month;"] # [doc = " assert_eq!(\"fEbruARy\".parse::<Month>(), Ok(Month::February));"] # [doc = " ```"] # [doc = ""] # [doc = " Only the shortest form (e.g. `jan`) and the longest form (e.g. `january`) is accepted."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::Month;"] # [doc = " assert!(\"septem\".parse::<Month>().is_err());"] # [doc = " assert!(\"Augustin\".parse::<Month>().is_err());"] # [doc = " ```"] impl FromStr for Month { type Err = ParseMonthError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if let Ok (("" , w)) = scan :: short_or_long_month0 (s) { match w { 0 => Ok (Month :: January) , 1 => Ok (Month :: February) , 2 => Ok (Month :: March) , 3 => Ok (Month :: April) , 4 => Ok (Month :: May) , 5 => Ok (Month :: June) , 6 => Ok (Month :: July) , 7 => Ok (Month :: August) , 8 => Ok (Month :: September) , 9 => Ok (Month :: October) , 10 => Ok (Month :: November) , 11 => Ok (Month :: December) , _ => Err (ParseMonthError { _dummy : () }) , } } else { Err (ParseMonthError { _dummy : () }) } } }
    };
}

impl_291!()