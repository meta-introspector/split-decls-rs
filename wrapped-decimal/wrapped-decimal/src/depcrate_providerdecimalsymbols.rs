// Generated macro for DecimalSymbols (struct)
macro_rules! Depcrate_providerDecimalSymbols {
() => {
// Module: crate::provider
// Provides: {"DecimalSymbols"}
// Dependencies: {}
# [doc = " Symbols and metadata required for formatting a [`Decimal`](crate::Decimal)."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_decimal :: provider))] pub struct DecimalSymbols < 'data > { # [doc = " String data for the symbols: +/- affixes and separators"] # [cfg_attr (feature = "serde" , serde (borrow))] pub strings : VarZeroCow < 'data , DecimalSymbolsStrs > , # [doc = " Settings used to determine where to place groups in the integer part of the number."] pub grouping_sizes : GroupingSizes , }
};
}
