// Generated macro for impl_7 (impl)
macro_rules! Depcrate_coreimpl_7 {
() => {
// Module: crate::core
// Provides: {"impl_7"}
// Dependencies: {}
impl EncodingType for Utf16 { type CodeUnit = u16 ; fn slice (source : & [Self :: CodeUnit] , start : usize , end : usize) -> Option < & [Self :: CodeUnit] > { source . get (start .. end) } fn get_ascii (source : & [Self :: CodeUnit] , index : usize) -> ParserResult < Option < u8 > > { source . get (index) . copied () . map (to_ascii_byte) . transpose () } fn check_calendar_key (key : & [Self :: CodeUnit]) -> bool { key == [0x75 , 0x2d , 0x63 , 0x61] } }
};
}
