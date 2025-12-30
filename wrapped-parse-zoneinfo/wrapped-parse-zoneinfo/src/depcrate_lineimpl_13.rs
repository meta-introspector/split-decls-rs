// Generated macro for impl_13 (impl)
macro_rules! Depcrate_lineimpl_13 {
() => {
// Module: crate::line
// Provides: {"impl_13"}
// Dependencies: {}
impl Weekday { fn calculate (year : i64 , month : Month , day : i8) -> Weekday { let m = month as i64 ; let y = if m < 3 { year - 1 } else { year } ; let d = day as i64 ; const T : [i64 ; 12] = [0 , 3 , 2 , 5 , 0 , 3 , 5 , 1 , 4 , 6 , 2 , 4] ; match (y + y / 4 - y / 100 + y / 400 + T [m as usize - 1] + d) % 7 { 0 => Weekday :: Sunday , 1 => Weekday :: Monday , 2 => Weekday :: Tuesday , 3 => Weekday :: Wednesday , 4 => Weekday :: Thursday , 5 => Weekday :: Friday , 6 => Weekday :: Saturday , _ => panic ! ("why is negative modulus designed so?") , } } }
};
}
