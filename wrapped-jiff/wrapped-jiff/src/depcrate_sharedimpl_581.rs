// Generated macro for impl_581 (impl)
macro_rules! Depcrate_sharedimpl_581 {
() => {
// Module: crate::shared
// Provides: {"impl_581"}
// Dependencies: {}
impl TzifDateTime { pub const ZERO : TzifDateTime = TzifDateTime :: new (0 , 0 , 0 , 0 , 0 , 0) ; pub const fn new (year : i16 , month : i8 , day : i8 , hour : i8 , minute : i8 , second : i8 ,) -> TzifDateTime { let mut bits = (year as u64) << 48 ; bits |= (month as u64) << 40 ; bits |= (day as u64) << 32 ; bits |= (hour as u64) << 24 ; bits |= (minute as u64) << 16 ; bits |= (second as u64) << 8 ; TzifDateTime { bits : bits as i64 } } pub const fn year (self) -> i16 { (self . bits as u64 >> 48) as u16 as i16 } pub const fn month (self) -> i8 { (self . bits as u64 >> 40) as u8 as i8 } pub const fn day (self) -> i8 { (self . bits as u64 >> 32) as u8 as i8 } pub const fn hour (self) -> i8 { (self . bits as u64 >> 24) as u8 as i8 } pub const fn minute (self) -> i8 { (self . bits as u64 >> 16) as u8 as i8 } pub const fn second (self) -> i8 { (self . bits as u64 >> 8) as u8 as i8 } }
};
}
