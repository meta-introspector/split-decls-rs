// Generated macro for impl_647 (impl)
macro_rules! Depcrate_provider_pattern_reference_parserimpl_647 {
() => {
// Module: crate::provider::pattern::reference::parser
// Provides: {"impl_647"}
// Dependencies: {}
impl SymbolAlias { fn try_new (ch : char) -> Option < Self > { matches ! (ch , 'Z') . then_some (Self { ch , length : 1 }) } fn finish (self , result : & mut Vec < PatternItem >) -> Result < () , PatternError > { match (self . ch , self . length) { ('Z' , 1 ..= 3) => SegmentSymbol { symbol : FieldSymbol :: TimeZone (TimeZone :: Iso) , length : 4 , } , ('Z' , 4) => SegmentSymbol { symbol : FieldSymbol :: TimeZone (TimeZone :: LocalizedOffset) , length : 4 , } , ('Z' , 5) => SegmentSymbol { symbol : FieldSymbol :: TimeZone (TimeZone :: IsoWithZ) , length : 5 , } , _ => return Err (PatternError :: UnknownSubstitution (self . ch)) , } . finish (result) } }
};
}
