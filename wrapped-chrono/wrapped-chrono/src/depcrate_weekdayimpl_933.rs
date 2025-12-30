// Generated macro for impl_933 (impl)
macro_rules! Depcrate_weekdayimpl_933 {
() => {
// Module: crate::weekday
// Provides: {"impl_933"}
// Dependencies: {}
impl fmt :: Display for Weekday { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . pad (match * self { Weekday :: Mon => "Mon" , Weekday :: Tue => "Tue" , Weekday :: Wed => "Wed" , Weekday :: Thu => "Thu" , Weekday :: Fri => "Fri" , Weekday :: Sat => "Sat" , Weekday :: Sun => "Sun" , }) } }
};
}
