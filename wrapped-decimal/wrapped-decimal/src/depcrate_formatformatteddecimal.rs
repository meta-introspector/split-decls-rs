// Generated macro for FormattedDecimal (struct)
macro_rules! Depcrate_formatFormattedDecimal {
() => {
// Module: crate::format
// Provides: {"FormattedDecimal"}
// Dependencies: {}
# [doc = " An intermediate structure returned by [`DecimalFormatter`](crate::DecimalFormatter)."] # [doc = " Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer."] # [derive (Debug , PartialEq , Clone)] pub struct FormattedDecimal < 'l > { pub (crate) value : & 'l Decimal , pub (crate) options : & 'l DecimalFormatterOptions , pub (crate) symbols : & 'l DecimalSymbols < 'l > , pub (crate) digits : & 'l [char ; 10] , }
};
}
