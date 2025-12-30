// Generated macro for impl_385 (impl)
macro_rules! Depcrate_parseimpl_385 {
() => {
// Module: crate::parse
// Provides: {"impl_385"}
// Dependencies: {}
impl < 'a > ParsedByteStr < 'a > { pub fn try_from_base64 (str : & ParsedStr < 'a >) -> Result < Self , base64 :: DecodeError > { let base64_str = match str { ParsedStr :: Allocated (string) => string . as_str () , ParsedStr :: Slice (str) => str , } ; base64 :: engine :: Engine :: decode (& base64 :: engine :: general_purpose :: STANDARD , base64_str) . map (ParsedByteStr :: Allocated) } }
};
}
