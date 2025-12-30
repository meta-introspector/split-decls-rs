// Generated macro for QuoteStyle (enum)
macro_rules! DepcrateQuoteStyle {
() => {
// Module: crate
// Provides: {"QuoteStyle"}
// Dependencies: {}
# [doc = " The quoting style to use when writing CSV data."] # [derive (Clone , Copy , Debug , Default)] # [non_exhaustive] pub enum QuoteStyle { # [doc = " This puts quotes around every field. Always."] Always , # [doc = " This puts quotes around fields only when necessary."] # [doc = ""] # [doc = " They are necessary when fields contain a quote, delimiter or record"] # [doc = " terminator. Quotes are also necessary when writing an empty record"] # [doc = " (which is indistinguishable from a record with one empty field)."] # [doc = ""] # [doc = " This is the default."] # [default] Necessary , # [doc = " This puts quotes around all fields that are non-numeric. Namely, when"] # [doc = " writing a field that does not parse as a valid float or integer, then"] # [doc = " quotes will be used even if they aren't strictly necessary."] NonNumeric , # [doc = " This *never* writes quotes, even if it would produce invalid CSV data."] Never , }
};
}
