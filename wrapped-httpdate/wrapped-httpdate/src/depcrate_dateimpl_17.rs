// Generated macro for impl_17 (impl)
macro_rules! Depcrate_dateimpl_17 {
() => {
// Module: crate::date
// Provides: {"impl_17"}
// Dependencies: {}
impl Display for HttpDate { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { let wday = match self . wday { 1 => b"Mon" , 2 => b"Tue" , 3 => b"Wed" , 4 => b"Thu" , 5 => b"Fri" , 6 => b"Sat" , 7 => b"Sun" , _ => unreachable ! () , } ; let mon = match self . mon { 1 => b"Jan" , 2 => b"Feb" , 3 => b"Mar" , 4 => b"Apr" , 5 => b"May" , 6 => b"Jun" , 7 => b"Jul" , 8 => b"Aug" , 9 => b"Sep" , 10 => b"Oct" , 11 => b"Nov" , 12 => b"Dec" , _ => unreachable ! () , } ; let mut buf : [u8 ; 29] = * b"   , 00     0000 00:00:00 GMT" ; buf [0] = wday [0] ; buf [1] = wday [1] ; buf [2] = wday [2] ; buf [5] = b'0' + (self . day / 10) ; buf [6] = b'0' + (self . day % 10) ; buf [8] = mon [0] ; buf [9] = mon [1] ; buf [10] = mon [2] ; buf [12] = b'0' + (self . year / 1000) as u8 ; buf [13] = b'0' + (self . year / 100 % 10) as u8 ; buf [14] = b'0' + (self . year / 10 % 10) as u8 ; buf [15] = b'0' + (self . year % 10) as u8 ; buf [17] = b'0' + (self . hour / 10) ; buf [18] = b'0' + (self . hour % 10) ; buf [20] = b'0' + (self . min / 10) ; buf [21] = b'0' + (self . min % 10) ; buf [23] = b'0' + (self . sec / 10) ; buf [24] = b'0' + (self . sec % 10) ; f . write_str (std :: str :: from_utf8 (& buf [..]) . unwrap ()) } }
};
}
