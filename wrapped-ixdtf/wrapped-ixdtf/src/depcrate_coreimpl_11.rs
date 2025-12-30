// Generated macro for impl_11 (impl)
macro_rules! Depcrate_coreimpl_11 {
() => {
// Module: crate::core
// Provides: {"impl_11"}
// Dependencies: {}
impl EncodingType for Utf8 { type CodeUnit = u8 ; fn slice < 'a > (source : & [Self :: CodeUnit] , start : usize , end : usize) -> Option < & [Self :: CodeUnit] > { source . get (start .. end) } fn get_ascii (source : & [Self :: CodeUnit] , index : usize) -> ParserResult < Option < u8 > > { Ok (source . get (index) . copied ()) } fn check_calendar_key (key : & [Self :: CodeUnit]) -> bool { key == "u-ca" . as_bytes () } }
};
}
