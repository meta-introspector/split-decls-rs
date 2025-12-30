// Generated macro for impl_32 (impl)
macro_rules! Depcrate_time_deltaimpl_32 {
() => {
// Module: crate::time_delta
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Display for TimeDelta { # [doc = " Format a `TimeDelta` using the [ISO 8601] format"] # [doc = ""] # [doc = " [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601#Durations"] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (abs , sign) = if self . secs < 0 { (- * self , "-") } else { (* self , "") } ; write ! (f , "{sign}P") ? ; if abs . secs == 0 && abs . nanos == 0 { return f . write_str ("0D") ; } f . write_fmt (format_args ! ("T{}" , abs . secs)) ? ; if abs . nanos > 0 { let mut figures = 9usize ; let mut fraction_digits = abs . nanos ; loop { let div = fraction_digits / 10 ; let last_digit = fraction_digits % 10 ; if last_digit != 0 { break ; } fraction_digits = div ; figures -= 1 ; } f . write_fmt (format_args ! (".{fraction_digits:0figures$}")) ? ; } f . write_str ("S") ? ; Ok (()) } }
};
}
