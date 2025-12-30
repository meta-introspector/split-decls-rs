// Generated macro for StdFmtWrite (struct)
macro_rules! Depcrate_fmtStdFmtWrite {
() => {
// Module: crate::fmt
// Provides: {"StdFmtWrite"}
// Dependencies: {}
# [doc = " An adapter for using `std::fmt::Write` implementations with `fmt::Write`."] # [doc = ""] # [doc = " This is useful when one wants to format a datetime or span directly"] # [doc = " to something with a `std::fmt::Write` trait implementation but not a"] # [doc = " `fmt::Write` implementation."] # [doc = ""] # [doc = " (Despite using `Std` in this name, this type is available in `core`-only"] # [doc = " configurations.)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows the `std::fmt::Display` trait implementation for"] # [doc = " [`civil::DateTime`](crate::civil::DateTime) (but using a wrapper type)."] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{civil::DateTime, fmt::{temporal::DateTimePrinter, StdFmtWrite}};"] # [doc = ""] # [doc = " struct MyDateTime(DateTime);"] # [doc = ""] # [doc = " impl std::fmt::Display for MyDateTime {"] # [doc = "     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {"] # [doc = ""] # [doc = "         static P: DateTimePrinter = DateTimePrinter::new();"] # [doc = "         P.print_datetime(&self.0, StdFmtWrite(f))"] # [doc = "             .map_err(|_| std::fmt::Error)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let dt = MyDateTime(DateTime::constant(2024, 6, 15, 17, 30, 0, 0));"] # [doc = " assert_eq!(dt.to_string(), \"2024-06-15T17:30:00\");"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct StdFmtWrite < W > (pub W) ;
};
}
