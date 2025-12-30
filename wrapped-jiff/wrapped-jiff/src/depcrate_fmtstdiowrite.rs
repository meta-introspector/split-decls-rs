// Generated macro for StdIoWrite (struct)
macro_rules! Depcrate_fmtStdIoWrite {
() => {
// Module: crate::fmt
// Provides: {"StdIoWrite"}
// Dependencies: {}
# [doc = " An adapter for using `std::io::Write` implementations with `fmt::Write`."] # [doc = ""] # [doc = " This is useful when one wants to format a datetime or span directly"] # [doc = " to something with a `std::io::Write` trait implementation but not a"] # [doc = " `fmt::Write` implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{fs::File, io::{BufWriter, Write}, path::Path};"] # [doc = ""] # [doc = " use jiff::{civil::date, fmt::{StdIoWrite, temporal::DateTimePrinter}};"] # [doc = ""] # [doc = " let zdt = date(2024, 6, 15).at(7, 0, 0, 0).in_tz(\"America/New_York\")?;"] # [doc = ""] # [doc = " let path = Path::new(\"/tmp/output\");"] # [doc = " let mut file = BufWriter::new(File::create(path)?);"] # [doc = " DateTimePrinter::new().print_zoned(&zdt, StdIoWrite(&mut file)).unwrap();"] # [doc = " file.flush()?;"] # [doc = " assert_eq!("] # [doc = "     std::fs::read_to_string(path)?,"] # [doc = "     \"2024-06-15T07:00:00-04:00[America/New_York]\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [cfg (feature = "std")] # [derive (Clone , Debug)] pub struct StdIoWrite < W > (pub W) ;
};
}
