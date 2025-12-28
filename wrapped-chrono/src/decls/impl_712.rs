macro_rules! deps {
    () => {
        WeekdaySet!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        # [doc = " Print the underlying bitmask, padded to 7 bits."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use chrono::WeekdaySet;"] # [doc = " use chrono::Weekday::*;"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::single(Mon)), \"WeekdaySet(0000001)\");"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::single(Tue)), \"WeekdaySet(0000010)\");"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::ALL), \"WeekdaySet(1111111)\");"] # [doc = " ```"] impl Debug for WeekdaySet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "WeekdaySet({:0>7b})" , self . 0) } }
    };
}

impl_712!();