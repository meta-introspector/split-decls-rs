macro_rules! deps {
    () => {
        Weekday!();
        WeekdaySet!();
    };
}

macro_rules! impl_719 {
    () => {
        deps!();
        # [doc = " Print the collection as a slice-like list of weekdays."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use chrono::WeekdaySet;"] # [doc = " use chrono::Weekday::*;"] # [doc = " assert_eq!(\"[]\", WeekdaySet::EMPTY.to_string());"] # [doc = " assert_eq!(\"[Mon]\", WeekdaySet::single(Mon).to_string());"] # [doc = " assert_eq!(\"[Mon, Fri, Sun]\", WeekdaySet::from_array([Mon, Fri, Sun]).to_string());"] # [doc = " ```"] impl fmt :: Display for WeekdaySet { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "[") ? ; let mut iter = self . iter (Weekday :: Mon) ; if let Some (first) = iter . next () { write ! (f , "{first}") ? ; } for weekday in iter { write ! (f , ", {weekday}") ? ; } write ! (f , "]") } }
    };
}

impl_719!();