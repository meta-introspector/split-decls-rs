// Generated macro for impl_450 (impl)
macro_rules! Depcrate_provider_fields_componentsimpl_450 {
() => {
// Module: crate::provider::fields::components
// Provides: {"impl_450"}
// Dependencies: {}
impl From < TimeZoneName > for Field { fn from (time_zone_name : TimeZoneName) -> Self { match time_zone_name { TimeZoneName :: ShortSpecific => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: SpecificNonLocation) , length : FieldLength :: One , } , TimeZoneName :: LongSpecific => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: SpecificNonLocation) , length : FieldLength :: Four , } , TimeZoneName :: LongOffset => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: LocalizedOffset) , length : FieldLength :: Four , } , TimeZoneName :: ShortOffset => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: LocalizedOffset) , length : FieldLength :: One , } , TimeZoneName :: ShortGeneric => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: GenericNonLocation) , length : FieldLength :: One , } , TimeZoneName :: LongGeneric => Field { symbol : FieldSymbol :: TimeZone (fields :: TimeZone :: GenericNonLocation) , length : FieldLength :: Four , } , } } }
};
}
