// Generated macro for FractionalPrinter (struct)
macro_rules! Depcrate_fmt_friendly_printerFractionalPrinter {
() => {
// Module: crate::fmt::friendly::printer
// Provides: {"FractionalPrinter"}
// Dependencies: {}
# [doc = " A printer for a fraction with an integer and fraction component."] # [doc = ""] # [doc = " This also includes the formatter for the integer component and the"] # [doc = " formatter for the fractional component."] struct FractionalPrinter { integer : u64 , fraction : u32 , fmtint : DecimalFormatter , fmtfraction : FractionalFormatter , }
};
}
