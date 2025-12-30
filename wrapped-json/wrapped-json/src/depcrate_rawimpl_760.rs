// Generated macro for impl_760 (impl)
macro_rules! Depcrate_rawimpl_760 {
() => {
// Module: crate::raw
// Provides: {"impl_760"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for ReferenceFromString { type Value = & 'de RawValue ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("raw value") } fn visit_borrowed_str < E > (self , s : & 'de str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (RawValue :: from_borrowed (s)) } }
};
}
