// Generated macro for impl_951 (impl)
macro_rules! Depcrate_weekday_setimpl_951 {
() => {
// Module: crate::weekday_set
// Provides: {"impl_951"}
// Dependencies: {}
# [doc = " Print the underlying bitmask, padded to 7 bits."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use chrono::WeekdaySet;"] # [doc = " use chrono::Weekday::*;"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::single(Mon)), \"WeekdaySet(0000001)\");"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::single(Tue)), \"WeekdaySet(0000010)\");"] # [doc = " assert_eq!(format!(\"{:?}\", WeekdaySet::ALL), \"WeekdaySet(1111111)\");"] # [doc = " ```"] impl Debug for WeekdaySet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "WeekdaySet({:0>7b})" , self . 0) } }
};
}
