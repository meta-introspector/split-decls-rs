// Generated macro for FormattedCompactDecimal (struct)
macro_rules! Depcrate_compactdecimal_formatFormattedCompactDecimal {
() => {
// Module: crate::compactdecimal::format
// Provides: {"FormattedCompactDecimal"}
// Dependencies: {}
# [doc = " An intermediate structure returned by [`CompactDecimalFormatter`](super::CompactDecimalFormatter)."] # [doc = " Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer."] # [derive (Debug)] pub struct FormattedCompactDecimal < 'l > { pub (crate) formatter : & 'l CompactDecimalFormatter , pub (crate) value : Cow < 'l , CompactDecimal > , pub (crate) plural_map : Option < ZeroMap2dCursor < 'l , 'l , i8 , Count , PatternULE > > , }
};
}
