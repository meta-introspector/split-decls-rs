// Generated macro for impl_821 (impl)
macro_rules! Depcrate_rawimpl_821 {
() => {
// Module: crate::raw
// Provides: {"impl_821"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for ReferenceFromString { type Value = & 'de RawValue ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("raw value") } fn visit_borrowed_str < E > (self , s : & 'de str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (RawValue :: from_borrowed (s)) } }
};
}
