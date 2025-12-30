// Generated macro for SizeFormat (enum)
macro_rules! Depcrate_output_tableSizeFormat {
() => {
// Module: crate::output::table
// Provides: {"SizeFormat"}
// Dependencies: {}
# [doc = " Formatting options for file sizes."] # [allow (clippy :: enum_variant_names)] # [derive (PartialEq , Eq , Debug , Default , Copy , Clone)] pub enum SizeFormat { # [doc = " Format the file size using **decimal** prefixes, such as “kilo”,"] # [doc = " “mega”, or “giga”."] # [default] DecimalBytes , # [doc = " Format the file size using **binary** prefixes, such as “kibi”,"] # [doc = " “mebi”, or “gibi”."] BinaryBytes , # [doc = " Do no formatting and just display the size as a number of bytes."] JustBytes , }
};
}
