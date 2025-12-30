// Generated macro for FormatSizeOptions (struct)
macro_rules! Depcrate_optionsFormatSizeOptions {
() => {
// Module: crate::options
// Provides: {"FormatSizeOptions"}
// Dependencies: {}
# [doc = " Holds the options for the `file_size` method."] # [derive (Debug , Clone , Copy , Default)] # [non_exhaustive] pub struct FormatSizeOptions { # [doc = " Whether the value being formatted represents an amount of bits or bytes."] pub base_unit : BaseUnit , # [doc = " The scale (binary/decimal) to divide against."] pub kilo : Kilo , # [doc = " The unit set to display."] pub units : Kilo , # [doc = " The amount of decimal places to display if the decimal part is non-zero."] pub decimal_places : usize , # [doc = " The amount of zeroes to display if the decimal part is zero."] pub decimal_zeroes : usize , # [doc = " Whether to force a certain representation and if so, which one."] pub fixed_at : Option < FixedAt > , # [doc = " Whether to use the full unit (e.g. `Kilobyte`) or its abbreviation (`kB`)."] pub long_units : bool , # [doc = " Whether to place a space between value and units."] pub space_after_value : bool , # [doc = " An optional suffix which will be appended after the unit. Useful to represent speeds (e.g. `1 kB/s`)"] pub suffix : & 'static str , pub thousands_separator : Option < char > , }
};
}
