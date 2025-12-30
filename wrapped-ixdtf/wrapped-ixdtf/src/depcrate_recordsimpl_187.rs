// Generated macro for impl_187 (impl)
macro_rules! Depcrate_recordsimpl_187 {
() => {
// Module: crate::records
// Provides: {"impl_187"}
// Dependencies: {}
impl UtcOffsetRecordOrZ { # [doc = " Resolves to a [`UtcOffsetRecord`] according to RFC9557: \"Z\" == \"-00:00\""] pub fn resolve_rfc_9557 (self) -> UtcOffsetRecord { match self { UtcOffsetRecordOrZ :: Offset (o) => o , UtcOffsetRecordOrZ :: Z => UtcOffsetRecord :: MinutePrecision (MinutePrecisionOffset { sign : Sign :: Negative , hour : 0 , minute : 0 , }) , } } }
};
}
